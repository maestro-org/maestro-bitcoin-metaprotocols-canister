use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::msg_caller;
use ic_cdk::management_canister::{HttpRequestResult, TransformArgs};
use ic_cdk_macros::*;
use ic_stable_structures::{DefaultMemoryImpl, StableCell, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeSet;

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

// Thread-local storage for the API key and request state
thread_local! {
    static API_KEY_STORAGE: RefCell<StableCell<ApiKey, DefaultMemoryImpl>> = RefCell::new(
        StableCell::init(
            DefaultMemoryImpl::default(),
            ApiKey { key: String::new() }
        ).unwrap()
    );

    // Track pending requests to prevent reentrancy
    static PENDING_REQUESTS: RefCell<BTreeSet<Principal>> = RefCell::new(BTreeSet::new());
}

// Guard for requests that modify state
pub struct CallerGuard {
    principal: Principal,
}

impl CallerGuard {
    pub fn new(principal: Principal) -> Result<Self, String> {
        PENDING_REQUESTS.with(|requests| {
            let mut pending = requests.borrow_mut();
            if pending.contains(&principal) {
                return Err(format!(
                    "Already processing a request for principal {:?}",
                    &principal
                ));
            }
            pending.insert(principal);
            Ok(Self { principal })
        })
    }
}

impl Drop for CallerGuard {
    fn drop(&mut self) {
        PENDING_REQUESTS.with(|requests| {
            requests.borrow_mut().remove(&self.principal);
        });
    }
}

// Guard function for authorization
fn authorized_guard() -> Result<(), String> {
    let caller = msg_caller();
    let caller_str = caller.to_text();

    if !AUTHORIZED_CALLERS.iter().any(|&auth| auth == caller_str) {
        return Err("Unauthorized: Caller not in authorized list".to_string());
    }
    Ok(())
}

#[query(guard = "authorized_guard")]
#[candid_method(query)]
fn get_api_key() -> Result<String, String> {
    // Authorization is handled by guard function
    API_KEY_STORAGE.with(|storage| {
        let key = storage.borrow().get().key.clone();
        if key.is_empty() {
            Err("API key not set".to_string())
        } else {
            Ok(key)
        }
    })
}

// Update the set_api_key function to use the global constant
#[update(guard = "authorized_guard")]
#[candid_method(update)]
async fn set_api_key(new_key: String) -> Result<(), String> {
    let caller = msg_caller();

    // Input validation
    if new_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    if new_key.len() > 1000 {
        return Err("API key too long (max 1000 characters)".to_string());
    }

    // Prevent reentrancy by the same caller
    let _guard = CallerGuard::new(caller).map_err(|e| format!("Reentrancy protection: {}", e))?;

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
