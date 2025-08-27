use candid::{candid_method, CandidType};
use ic_cdk::api::canister_self;
use ic_cdk::management_canister::{
    http_request, HttpHeader, HttpMethod, HttpRequestArgs, TransformContext, TransformFunc,
};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

use crate::common::{
    check_authorization, LastUpdated, MaestroCollectionStatsResponse,
    MaestroInscriptionInfoResponse, MaestroOmbColorGroup, BASE_URL,
};

// Address-specific types
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscriptionsResponse {
    pub data: Vec<MaestroAddressInscription>,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroAddressInscription {
    pub inscription_id: String,
    pub satoshis: String,
    pub utxo_sat_offset: i64,
    pub utxo_txid: String,
    pub utxo_vout: i32,
    pub utxo_block_height: i64,
    pub utxo_confirmations: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscription {
    pub inscription_id: String,
    pub satoshis: String,
    pub utxo_sat_offset: i64,
    pub utxo_txid: String,
    pub utxo_vout: i32,
    pub utxo_block_height: i64,
    pub utxo_confirmations: i64,
    pub collection_symbol: Option<String>,
    pub floor_price: i64,
    pub omb_color: Option<String>,
    pub omb_floor_price: Option<i64>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AddressInscriptions {
    pub data: Vec<AddressInscription>,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[update]
#[candid_method(update)]
pub async fn get_address_inscriptions(
    address: String,
    count: String,
) -> Result<AddressInscriptions, String> {
    check_authorization()?;

    let api_key = crate::get_api_key()?;

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
