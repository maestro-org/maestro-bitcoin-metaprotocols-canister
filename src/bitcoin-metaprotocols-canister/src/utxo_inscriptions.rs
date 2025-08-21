use candid::{candid_method, CandidType};
use ic_cdk::api::canister_self;
use ic_cdk::management_canister::{
    http_request, HttpHeader, HttpMethod, HttpRequestArgs, TransformContext, TransformFunc,
};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

use crate::common::{
    check_authorization, BASE_URL, LastUpdated, MaestroInscriptionInfoResponse, MaestroOmbColorGroup,
};

// UTXO-specific types
#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroTxOutIntoResponse {
    pub data: MaestroTxOut,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroTxOut {
    pub address: Option<String>,
    pub script_pubkey: String,
    pub satoshis: String,
    pub spending_tx: Option<String>,
    pub inscriptions: Vec<MaestroInscriptionAndOffset>,
    pub runes: Vec<MaestroRuneAndAmount>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroInscriptionAndOffset {
    pub inscription_id: String,
    pub offset: i64,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct MaestroRuneAndAmount {
    pub rune_id: String,
    pub amount: String,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct UtxoInscription {
    pub inscription_id: String,
    pub collection_symbol: Option<String>,
    pub omb_color: Option<String>,
    pub omb_floor_price: Option<i64>,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct UtxoInscriptions {
    pub data: Vec<UtxoInscription>,
    pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}

#[update]
#[candid_method(update)]
pub async fn get_utxo_inscriptions(
    tx_hash: String,
    output_index: String,
) -> Result<UtxoInscriptions, String> {
    check_authorization()?;

    let api_key = crate::get_api_key();

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
