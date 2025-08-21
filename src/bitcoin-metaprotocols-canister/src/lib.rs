use candid::{candid_method, CandidType};
use ic_cdk::api::msg_caller;
use ic_cdk::management_canister::{HttpRequestResult, TransformArgs};
use ic_cdk_macros::*;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;

mod address_inscriptions;
mod common;
mod utxo_inscriptions;

// Re-export public functions and types to maintain the same API
pub use address_inscriptions::{get_address_inscriptions, AddressInscription, AddressInscriptions};
pub use common::{LastUpdated, AUTHORIZED_CALLERS};
pub use utxo_inscriptions::{get_utxo_inscriptions, UtxoInscription, UtxoInscriptions};

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
struct ApiKey {
    key: String,
}

impl Storable for ApiKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(serde_json::to_vec(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;
}

// Thread-local storage for the API key
thread_local! {
    static API_KEY_STORAGE: RefCell<StableCell<ApiKey, DefaultMemoryImpl>> = RefCell::new(
        StableCell::init(
            DefaultMemoryImpl::default(),
            ApiKey { key: String::new() }
        ).unwrap()
    );
}

#[query]
#[candid_method(query)]
fn get_api_key() -> String {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        panic!("Unauthorized");
    }

    API_KEY_STORAGE.with(|storage| storage.borrow().get().key.clone())
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

    API_KEY_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .set(ApiKey { key: new_key })
            .map(|_| ())
            .map_err(|e| format!("Failed to save API key: {:?}", e))
    })
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
