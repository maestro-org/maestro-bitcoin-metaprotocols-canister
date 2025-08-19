use candid::{candid_method, CandidType};
use ic_cdk::api::{canister_self, msg_caller};
use ic_cdk::management_canister::{
    http_request, HttpHeader, HttpMethod, HttpRequestArgs, HttpRequestResult, TransformArgs,
    TransformContext, TransformFunc,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

// General
const AUTHORIZED_CALLERS: [&str; 7] = [
    "62ick-jmsqq-h6wq5-emdfw-qblno-qphae-hs7y3-dxoyp-xiccq-bw4q3-aae", // maestro
    "xktoe-jjqeb-tzsr3-hxjir-en65h-6agv7-bbq2g-dyoch-276wj-waea7-rqe",
    "roqha-4aaaa-aaaap-qplnq-cai", // liquidium
    "e453p-eqaaa-aaaar-qanya-cai",
    "vr4ua-siaaa-aaaar-qaosq-cai",
    "pimqm-2dtug-w3ejt-krqai-jlp3u-uux2y-erjcw-wbvhu-pmvhu-hunju-wqe",
    "daoh3-exchb-6dvbd-fyxld-7kxjo-fdddf-4vhqp-mcoo2-s7gqh-qwpfd-pae",
];

const BASE_URL: &str = "https://xbt-mainnet.gomaestro-api.org/v0";

#[derive(CandidType, Deserialize, Serialize, Debug)]
struct ApiKey {
    key: String,
}

#[query]
#[candid_method(query)]
fn get_api_key() -> String {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        panic!("Unauthorized");
    }

    let (api_key,): (ApiKey,) = storage::stable_restore().expect("Failed to load API key");
    api_key.key
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct LastUpdated {
    block_hash: String,
    block_height: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfo {
    collection_symbol: Option<String>,
}

// Address Inscriptions
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscriptionsResponse {
    data: Vec<MaestroAddressInscription>,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscription {
    inscription_id: String,
    satoshis: String,
    utxo_sat_offset: i64,
    utxo_txid: String,
    utxo_vout: i32,
    utxo_block_height: i64,
    utxo_confirmations: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionInfoResponse {
    data: MaestroInscriptionInfo,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroCollectionStats {
    floor_price: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroCollectionStatsResponse {
    data: MaestroCollectionStats,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscription {
    inscription_id: String,
    satoshis: String,
    utxo_sat_offset: i64,
    utxo_txid: String,
    utxo_vout: i32,
    utxo_block_height: i64,
    utxo_confirmations: i64,
    collection_symbol: Option<String>,
    floor_price: i64,
    omb_color: Option<String>,
    omb_floor_price: Option<i64>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscriptions {
    data: Vec<AddressInscription>,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

// UTXO Inscriptions
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroTxOutIntoResponse {
    data: MaestroTxOut,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroTxOut {
    address: Option<String>,
    script_pubkey: String,
    satoshis: String,
    spending_tx: Option<String>,
    inscriptions: Vec<MaestroInscriptionAndOffset>,
    runes: Vec<MaestroRuneAndAmount>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionAndOffset {
    inscription_id: String,
    offset: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroRuneAndAmount {
    rune_id: String,
    amount: String,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct UtxoInscription {
    inscription_id: String,
    collection_symbol: Option<String>,
    omb_color: Option<String>,
    omb_floor_price: Option<i64>,
}
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroOmbColorGroupData {
    #[serde(rename = "color")]
    omb_color: String,
    #[serde(rename = "floor_price")]
    omb_floor_price: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroOmbColorGroup {
    data: MaestroOmbColorGroupData,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct UtxoInscriptions {
    data: Vec<UtxoInscription>,
    last_updated: LastUpdated,
    next_cursor: Option<String>,
}

#[update]
#[candid_method(update)]
async fn get_address_inscriptions(
    address: String,
    count: String,
) -> Result<AddressInscriptions, String> {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        return Err("Unauthorized".into());
    }

    let api_key = get_api_key();

    let address_inscriptions_maestro_url = format!(
        "{}/addresses/{}/inscriptions?count={}",
        BASE_URL, address, count
    );

    let address_inscriptions_maestro_request = HttpRequestArgs {
        url: address_inscriptions_maestro_url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "api-key".to_string(),
            value: api_key.clone(),
        }],
        body: None,
        max_response_bytes: Some(5 * 1000), // 5000 KB
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
    };

    match http_request(&address_inscriptions_maestro_request).await {
        Ok(response) => {
            let raw_body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("HTTP response body: {}", raw_body);

            let address_inscriptions_maestro_response: MaestroAddressInscriptionsResponse =
                serde_json::from_slice(&response.body)
                    .map_err(|e| format!("Failed to parse: {} (body: {})", e, raw_body))?;

            let mut final_result: Vec<AddressInscription> = Vec::new();

            for inscription in address_inscriptions_maestro_response.data {
                // Fetch inscription info
                let inscription_info_url = format!(
                    "{}/assets/inscriptions/{}",
                    BASE_URL, inscription.inscription_id
                );

                let inscription_info_request = HttpRequestArgs {
                    url: inscription_info_url,
                    method: HttpMethod::GET,
                    headers: vec![HttpHeader {
                        name: "api-key".to_string(),
                        value: api_key.clone(),
                    }],
                    body: None,
                    max_response_bytes: Some(5 * 1000),
                    transform: Some(TransformContext {
                        function: TransformFunc::new(canister_self(), "transform".to_string()),
                        context: vec![],
                    }),
                };

                let collection_symbol = match http_request(&inscription_info_request).await {
                    Ok(inscription_info_response) => {
                        match serde_json::from_slice::<MaestroInscriptionInfoResponse>(
                            &inscription_info_response.body,
                        ) {
                            Ok(info_response) => info_response.data.collection_symbol,
                            Err(e) => {
                                ic_cdk::println!(
                                    "Failed to parse MaestroInscriptionInfoResponse: {} (body: {})",
                                    e,
                                    String::from_utf8_lossy(&inscription_info_response.body)
                                );
                                None
                            }
                        }
                    }
                    Err(e) => {
                        ic_cdk::println!(
                            "Failed to fetch inscription info for {}: {:?}",
                            inscription.inscription_id,
                            e
                        );
                        None
                    }
                };

                // Fetch floor price if collection_symbol exists
                let mut floor_price = 0;
                if let Some(ref symbol) = collection_symbol {
                    let collection_stats_url =
                        format!("{}/assets/collections/{}/stats", BASE_URL, symbol);

                    let collection_stats_request = HttpRequestArgs {
                        url: collection_stats_url,
                        method: HttpMethod::GET,
                        headers: vec![HttpHeader {
                            name: "api-key".to_string(),
                            value: api_key.clone(),
                        }],
                        body: None,
                        max_response_bytes: Some(5 * 1000),
                        transform: Some(TransformContext {
                            function: TransformFunc::new(canister_self(), "transform".to_string()),
                            context: vec![],
                        }),
                    };

                    floor_price = match http_request(&collection_stats_request).await {
                        Ok(collection_stats_response) => {
                            match serde_json::from_slice::<MaestroCollectionStatsResponse>(
                                &collection_stats_response.body,
                            ) {
                                Ok(stats_response) => stats_response
                                    .data
                                    .floor_price
                                    .unwrap_or("0".to_string())
                                    .parse::<i64>()
                                    .unwrap_or(0),
                                Err(e) => {
                                    ic_cdk::println!(
                                "Failed to parse MaestroCollectionStatsResponse: {} (body: {})",
                                e,
                                String::from_utf8_lossy(&collection_stats_response.body)
                            );
                                    0
                                }
                            }
                        }
                        Err(e) => {
                            ic_cdk::println!(
                                "Failed to fetch collection stats for {}: {:?}",
                                symbol,
                                e
                            );
                            0
                        }
                    };
                }

                // Fetch OMB color group
                let omb_color_group_url = format!(
                    "{}/assets/inscriptions/{}/omb_color_group",
                    BASE_URL, inscription.inscription_id
                );
                let omb_color_group_request = HttpRequestArgs {
                    url: omb_color_group_url,
                    method: HttpMethod::GET,
                    headers: vec![HttpHeader {
                        name: "api-key".to_string(),
                        value: api_key.clone(),
                    }],
                    body: None,
                    max_response_bytes: Some(5 * 1000),
                    transform: Some(TransformContext {
                        function: TransformFunc::new(canister_self(), "transform".to_string()),
                        context: vec![],
                    }),
                };
                let (omb_color, omb_floor_price) = match http_request(&omb_color_group_request)
                    .await
                {
                    Ok(omb_response) => {
                        match serde_json::from_slice::<MaestroOmbColorGroup>(&omb_response.body) {
                            Ok(omb) => (Some(omb.data.omb_color), Some(omb.data.omb_floor_price)),
                            Err(_) => (None, None),
                        }
                    }
                    Err(_) => (None, None),
                };

                final_result.push(AddressInscription {
                    inscription_id: inscription.inscription_id,
                    satoshis: inscription.satoshis,
                    utxo_sat_offset: inscription.utxo_sat_offset,
                    utxo_txid: inscription.utxo_txid,
                    utxo_vout: inscription.utxo_vout,
                    utxo_block_height: inscription.utxo_block_height,
                    utxo_confirmations: inscription.utxo_confirmations,
                    collection_symbol,
                    floor_price,
                    omb_color,
                    omb_floor_price,
                });
            }

            Ok(AddressInscriptions {
                data: final_result,
                last_updated: address_inscriptions_maestro_response.last_updated,
                next_cursor: address_inscriptions_maestro_response.next_cursor,
            })
        }
        Err(e) => Err(format!("HTTP error: {:?}", e)),
    }
}

#[update]
#[candid_method(update)]
async fn get_utxo_inscriptions(
    tx_hash: String,
    output_index: String,
) -> Result<UtxoInscriptions, String> {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        return Err("Unauthorized".into());
    }

    let api_key = get_api_key();

    let utxo_inscriptions_maestro_url = format!(
        "{}/transactions/{}/outputs/{}",
        BASE_URL, tx_hash, output_index
    );

    let utxo_inscriptions_maestro_request = HttpRequestArgs {
        url: utxo_inscriptions_maestro_url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "api-key".to_string(),
            value: api_key.clone(),
        }],
        body: None,
        max_response_bytes: Some(5 * 1000), // 5000 KB
        transform: Some(TransformContext {
            function: TransformFunc::new(canister_self(), "transform".to_string()),
            context: vec![],
        }),
    };

    match http_request(&utxo_inscriptions_maestro_request).await {
        Ok(response) => {
            let raw_body = String::from_utf8_lossy(&response.body);
            ic_cdk::println!("HTTP response body: {}", raw_body);

            let maestro_tx_out_into_response: MaestroTxOutIntoResponse =
                serde_json::from_slice(&response.body)
                    .map_err(|e| format!("Failed to parse: {} (body: {})", e, raw_body))?;

            let mut final_result: Vec<UtxoInscription> = Vec::new();

            for inscription in maestro_tx_out_into_response.data.inscriptions {
                let inscription_info_url = format!(
                    "{}/assets/inscriptions/{}",
                    BASE_URL, inscription.inscription_id
                );

                let inscription_info_request = HttpRequestArgs {
                    url: inscription_info_url,
                    method: HttpMethod::GET,
                    headers: vec![HttpHeader {
                        name: "api-key".to_string(),
                        value: api_key.clone(),
                    }],
                    body: None,
                    max_response_bytes: Some(5 * 1000),
                    transform: Some(TransformContext {
                        function: TransformFunc::new(canister_self(), "transform".to_string()),
                        context: vec![],
                    }),
                };

                let collection_symbol = match http_request(&inscription_info_request).await {
                    Ok(inscription_info_response) => {
                        match serde_json::from_slice::<MaestroInscriptionInfoResponse>(
                            &inscription_info_response.body,
                        ) {
                            Ok(info_response) => info_response.data.collection_symbol,
                            Err(e) => {
                                ic_cdk::println!(
                                    "Failed to parse MaestroInscriptionInfoResponse: {} (body: {})",
                                    e,
                                    String::from_utf8_lossy(&inscription_info_response.body)
                                );
                                None
                            }
                        }
                    }
                    Err(e) => {
                        ic_cdk::println!(
                            "Failed to fetch inscription info for {}: {:?}",
                            inscription.inscription_id,
                            e
                        );
                        None
                    }
                };

                // Fetch OMB color group
                let omb_color_group_url = format!(
                    "{}/assets/inscriptions/{}/omb_color_group",
                    BASE_URL, inscription.inscription_id
                );
                let omb_color_group_request = HttpRequestArgs {
                    url: omb_color_group_url,
                    method: HttpMethod::GET,
                    headers: vec![HttpHeader {
                        name: "api-key".to_string(),
                        value: api_key.clone(),
                    }],
                    body: None,
                    max_response_bytes: Some(5 * 1000),
                    transform: Some(TransformContext {
                        function: TransformFunc::new(canister_self(), "transform".to_string()),
                        context: vec![],
                    }),
                };
                let (omb_color, omb_floor_price) = match http_request(&omb_color_group_request)
                    .await
                {
                    Ok(omb_response) => {
                        match serde_json::from_slice::<MaestroOmbColorGroup>(&omb_response.body) {
                            Ok(omb) => (Some(omb.data.omb_color), Some(omb.data.omb_floor_price)),
                            Err(_) => (None, None),
                        }
                    }
                    Err(_) => (None, None),
                };

                final_result.push(UtxoInscription {
                    inscription_id: inscription.inscription_id,
                    collection_symbol,
                    omb_color,
                    omb_floor_price,
                });
            }

            Ok(UtxoInscriptions {
                data: final_result,
                last_updated: maestro_tx_out_into_response.last_updated,
                next_cursor: maestro_tx_out_into_response.next_cursor,
            })
        }
        Err(e) => Err(format!("HTTP error: {:?}", e)),
    }
}

// Update the set_api_key function to use the global constant
#[update]
#[candid_method(update)]
async fn set_api_key(new_key: String) -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        return Err("Unauthorized".into());
    }

    storage::stable_save((ApiKey { key: new_key },)).expect("Failed to save API key");

    Ok(())
}

#[ic_cdk::query(hidden = true)]
fn transform(raw: TransformArgs) -> HttpRequestResult {
    let headers = vec![];

    let mut res = HttpRequestResult {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == 200u8 {
        res.body = raw.response.body;
    } else {
        ic_cdk::println!("Received an error from maestro: err = {:?}", raw);
    }
    res
}

ic_cdk::export_candid!();
