use candid::CandidType;
use ic_cdk::api::msg_caller;
use serde::{Deserialize, Serialize};

// Constants
pub const AUTHORIZED_CALLERS: [&str; 7] = [
    "62ick-jmsqq-h6wq5-emdfw-qblno-qphae-hs7y3-dxoyp-xiccq-bw4q3-aae", // maestro
    "xktoe-jjqeb-tzsr3-hxjir-en65h-6agv7-bbq2g-dyoch-276wj-waea7-rqe",
    "roqha-4aaaa-aaaap-qplnq-cai", // liquidium
    "e453p-eqaaa-aaaar-qanya-cai",
    "vr4ua-siaaa-aaaar-qaosq-cai",
    "pimqm-2dtug-w3ejt-krqai-jlp3u-uux2y-erjcw-wbvhu-pmvhu-hunju-wqe",
    "daoh3-exchb-6dvbd-fyxld-7kxjo-fdddf-4vhqp-mcoo2-s7gqh-qwpfd-pae",
];

pub const BASE_URL: &str = "https://xbt-mainnet.gomaestro-api.org/v0";

// Common types
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct LastUpdated {
    pub block_hash: String,
    pub block_height: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfo {
    pub collection_symbol: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfoResponse {
    pub data: MaestroInscriptionInfo,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroCollectionStats {
    pub floor_price: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroCollectionStatsResponse {
    pub data: MaestroCollectionStats,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroOmbColorGroupData {
    #[serde(rename = "color")]
    pub omb_color: String,
    #[serde(rename = "floor_price")]
    pub omb_floor_price: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroOmbColorGroup {
    pub data: MaestroOmbColorGroupData,
}

// Utility functions
pub fn check_authorization() -> Result<(), String> {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        return Err("Unauthorized".into());
    }
    Ok(())
}
