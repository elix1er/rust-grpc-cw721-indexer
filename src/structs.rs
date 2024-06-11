use cosmos_sdk_proto::cosmwasm::wasm::v1::{AbsoluteTxPosition, ContractInfo};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AbsoluteTxPositionDef {
    block_height: u64,
    tx_index: u64,
}

impl From<AbsoluteTxPosition> for AbsoluteTxPositionDef {
    fn from(def: AbsoluteTxPosition) -> Self {
        Self {
            block_height: def.block_height,
            tx_index: def.tx_index,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AnyDef {
    type_url: String,
    value: Vec<u8>,
}

impl From<prost_types::Any> for AnyDef {
    fn from(any: prost_types::Any) -> Self {
        Self {
            type_url: any.type_url,
            value: any.value,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ContractInfoDef {
    code_id: u64,
    creator: String,
    admin: String,
    label: String,
    #[serde(with = "serde_with::rust::double_option")]
    created: Option<Option<AbsoluteTxPositionDef>>,
    ibc_port_id: String,
    #[serde(with = "serde_with::rust::double_option")]
    extension: Option<Option<AnyDef>>,
}

impl From<ContractInfo> for ContractInfoDef {
    fn from(info: ContractInfo) -> Self {
        Self {
            code_id: info.code_id,
            creator: info.creator,
            admin: info.admin,
            label: info.label,
            created: info.created.map(|pos| Some(pos.into())),
            ibc_port_id: info.ibc_port_id,
            extension: info.extension.map(|ext| Some(ext.into())),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CachedData {
    pub last_page: u64,
    pub data: Vec<ContractInfoDef>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContractDetails {
    pub contract_address: String,
    pub code_id: String,
}