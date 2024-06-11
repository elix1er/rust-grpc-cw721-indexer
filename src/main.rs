mod structs;

use tokio::task;
use tonic::transport::Uri;
use tonic::{transport::Channel, Request};
use log::{error, info, warn};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::join_all;
use cosmos_sdk_proto::cosmwasm::wasm::v1::ContractInfo;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{service_client::ServiceClient, GetTxsEventRequest};
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    query_client::QueryClient, QueryContractInfoRequest,
};

use crate::structs::{CachedData, ContractDetails, ContractInfoDef};

async fn get_cached_data() -> io::Result<CachedData> {
    match File::open("cached_data.json") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let cached_data: CachedData = serde_json::from_reader(reader)?;
            info!("Cached data loaded successfully.");
            Ok(cached_data)
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            warn!("Cached data file not found, initializing new data.");
            Ok(CachedData {
                last_page: 0,
                data: vec![],
            })
        }
        Err(e) => {
            error!("Failed to read cached data: {}", e);
            Err(e)
        }
    }
}

async fn append_cached_data(new_data: &CachedData) -> io::Result<()> {
    let file = File::create("cached_data.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, new_data).map_err(|e| {
        error!("Failed to write cached data: {}", e);
        io::Error::new(io::ErrorKind::Other, "Failed to write cached data")
    })?;
    info!("Cached data updated successfully.");
    Ok(())
}

async fn fetch_cw721_contracts(
    lcd_client: &mut ServiceClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("[RustyCw721Indexer]: API starts fetching initialized");

    let mut cached_data = get_cached_data().await?;
    info!("[RustyCw721Indexer]: Starting from page {:?}", cached_data.last_page);

    let mut has_more = true;
    let per_page = 100;
    let mut current_page = cached_data.last_page;

    let interacted_contracts_query = vec!["message.action='/cosmwasm.wasm.v1.MsgInstantiateContract'".to_string()];

    while has_more {
        let request = Request::new(GetTxsEventRequest {
            events: interacted_contracts_query.clone(),
            pagination: None,
            order_by: 0,
            page: current_page,
            limit: per_page,
        });

        let response = lcd_client.get_txs_event(request).await?;
        let response_inner = response.into_inner();

        let tx_responses = response_inner.tx_responses;
        let total = response_inner.total;

        info!("Processing transactions: {:?}", tx_responses);

        if tx_responses.is_empty() {
            has_more = false;
            info!("Fetch complete: No more transactions to process.");
        } else {
            current_page += 1;
            let total_pages = (total as f64 / per_page as f64).ceil() as u64;
            has_more = current_page < total_pages;

            info!("Processing page {} of {}", current_page, total_pages);

            let contract_details: Vec<ContractDetails> = tx_responses
                .iter()
                .flat_map(parse_initialized_contract_tx_result)
                .collect();

            let contract_addresses: Vec<String> = contract_details
                .iter()
                .map(|details| details.contract_address.clone())
                .collect();

            let raw_cw721s = batch_contract_query(contract_addresses).await;

            print!("{:?}", raw_cw721s);

            cached_data.data.extend(
                raw_cw721s
                    .into_iter()
                    .flat_map(|result| result.ok().map(|info| ContractInfoDef::from(info))),
            );
            cached_data.last_page = current_page;

            append_cached_data(&cached_data).await?;
        }
    }

    info!("Contract fetch process completed.");
    Ok(())
}

fn parse_initialized_contract_tx_result(
    tx_response: &cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse,
) -> Vec<ContractDetails> {
    tx_response
        .logs
        .iter()
        .flat_map(|log| log.events.iter())
        .filter(|event| event.r#type == "instantiate")
        .map(|event| {
            let mut contract_address = String::new();
            let mut code_id = String::new();
            for attr in &event.attributes {
                if attr.key == "_contract_address" {
                    contract_address = attr.value.clone();
                } else if attr.key == "code_id" {
                    code_id = attr.value.clone();
                }
            }
            ContractDetails {
                contract_address,
                code_id,
            }
        })
        .collect()
}

async fn batch_contract_query(addresses: Vec<String>) -> Vec<Result<ContractInfo, String>> {
    let endpoint = Channel::builder(Uri::from_static("https://grpc.juno.basementnodes.ca:443"));
    let channel = match endpoint.connect().await {
        Ok(channel) => channel,
        Err(err) => return vec![Err(format!("Failed to connect: {}", err))],
    };

    let q_client = Arc::new(Mutex::new(QueryClient::new(channel)));
    let tasks: Vec<_> = addresses.into_iter().map(|address| {
        let q_client = Arc::clone(&q_client);
        task::spawn(async move {
            let result: Result<ContractInfo, String> = match q_client
                .lock()
                .await
                .contract_info(QueryContractInfoRequest { address: address.clone() })
                .await
            {
                Ok(response) => {
                    if let Some(q_info) = response.into_inner().contract_info {
                        Ok(q_info)
                    } else {
                        Err(format!("No contract info found for address: {}", address))
                    }
                }
                Err(err) => Err(format!("Query failed: {}", err)),
            };
            result
        })
    }).collect();

    let results = join_all(tasks).await;
    results.into_iter().map(|res| res.unwrap()).collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Channel::builder(Uri::from_static("https://grpc.juno.basementnodes.ca:443"));
    let channel = endpoint.connect().await?;
    let mut lcd_client = ServiceClient::new(channel);

    fetch_cw721_contracts(&mut lcd_client).await?;
    Ok(())
}
