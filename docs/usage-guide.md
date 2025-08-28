# Bitcoin Metaprotocols Canister Usage Guide

## Overview

The Bitcoin Metaprotocols Canister is an [Internet Computer](https://internetcomputer.org) (ICP) canister that provides indexing services for Bitcoin metaprotocols, specifically focused on Bitcoin inscriptions. It leverages the [Maestro API](https://docs.gomaestro.org/bitcoin) to fetch and process Bitcoin metaprotocol data, providing structured access to inscription information associated with Bitcoin addresses and UTXOs.

Maestro Official Deployed Canister: [iayqr-yaaaa-aaaar-qbopq-cai](https://dashboard.internetcomputer.org/canister/iayqr-yaaaa-aaaar-qbopq-cai)

## Key Features

-   **Address Inscriptions**: Get all inscriptions associated with a Bitcoin address
-   **UTXO Inscriptions**: Get inscriptions for specific transaction outputs
-   **Collection Metadata**: Fetch collection symbols and floor prices
-   **Authorization**: Built-in access control for authorized callers only

## Prerequisites

-   A Maestro [API key](https://dashboard.gomaestro.org/login)

Before deploying and using the canister, ensure you have:

1. **Rust** - [Install Rust](https://www.rust-lang.org/tools/install)
2. **DFX** - [Install DFX](https://internetcomputer.org/docs/building-apps/getting-started/install#installing-dfx-via-dfxvm)
3. **WebAssembly target** for Rust:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
4. **Additional tools** for Candid generation:
    - `didc` binary from [Candid releases](https://github.com/dfinity/candid/releases)
    - `ic-wasm`: `cargo install ic-wasm`
    - `candid-extractor`: `cargo install candid-extractor`

### Authorization

The canister implements access control through a hardcoded list of authorized principals. As it currently exists, only the following principals can call the canister methods:

-   Maestro principals
-   Liquidium principals
-   Other authorized entities

**Current authorized callers**:

_These should be replaced with your own list of authorized callers before deployment and canister interaction._

```rust
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
```

[Source](https://github.com/maestro-org/maestro-bitcoin-metaprotocols-canister/blob/main/src/bitcoin-metaprotocols-canister/src/common.rs)

## Deployment Guide

### Local Development Deployment

**Note:** If the `--network` argument is not provided, it defaults to the public playground. For local deployments use `--network=local`. For mainnet use `--network=ic`.

#### 1. Setup identity (Optional)

Create a new dedicated identity, or:

Use the default one for local development:

```bash
dfx identity use default
```

#### 2. Start Local ICP Subnet

In a _separate_ terminal window, start the local Internet Computer subnet.

This will create a local canister execution environment and web server processes. This enables you to test your dapps during development.

```bash
dfx start --clean
```

Output:

```bash
Running dfx start for version 0.26.1
Using the default configuration for the local shared network.
Replica API running on 127.0.0.1:4943. You must open a new terminal to continue developing. If you'd prefer to stop, quit with 'Ctrl-C'.
```

#### 3. Create and Deploy Canister

Create the development canister:

_Creates an empty canister and associates the assigned Canister ID to the canister name._

```bash
dfx canister create bitcoin-metaprotocols-canister-dev
```

Output:

```bash
Created a wallet canister on the "local" network for user "default" with ID "uqqxf-5h777-77774-qaaaa-cai"
bitcoin-metaprotocols-canister-dev canister created with canister id: uxrrr-q7777-77774-qaaaq-cai
```

Generate DID:

-   A `.did` file is a text file that contains a [Candid](https://internetcomputer.org/docs/building-apps/interact-with-canisters/candid) service description, written either manually or generated from a canister's code.
-   It describes the public methods, arguments, and return types of a canister.

```bash
dfx generate --network=local bitcoin-metaprotocols-canister-dev
```

Output:

```bash
Generated type declarations for canister 'bitcoin-metaprotocols-canister-dev' to 'maestro-bitcoin-metaprotocols-canister/src/declarations/bitcoin-metaprotocols-canister-dev'
```

Build the canister:

_Compiles the program code into a WebAssembly module that can be deployed on ICP._

```bash
dfx build --network=local bitcoin-metaprotocols-canister-dev
```

Output:

```bash
Building canister 'bitcoin-metaprotocols-canister-dev'.
Executing: cargo build --target wasm32-unknown-unknown --release -p bitcoin-metaprotocols-canister --locked
Finished building canisters.
```

Install the canister:

_Installs compiled code in a canister._

```bash
dfx canister install --network=local bitcoin-metaprotocols-canister-dev
```

Output:

```bash
Installed code for canister bitcoin-metaprotocols-canister-dev, with canister ID uxrrr-q7777-77774-qaaaq-cai
```

**Note:** You can also run `dfx canister deploy` to combine the following steps in the future:

-   `dfx canister create <canister_name>`
-   `dfx build`
-   `dfx canister install <canister_name>`

#### 4. Render the Canister

After [starting the local ICP subnet](#2-start-local-icp-subnet), we can leverage [Candid UI](https://internetcomputer.org/docs/building-apps/interact-with-canisters/candid/using-candid) to interact with our canister directly within the browser.

```bash
http://127.0.0.1:4943/?canisterId=u6s2n-gx777-77774-qaaba-cai&id=uxrrr-q7777-77774-qaaaq-cai
```

You may notice the `canisterId` query parameter with value: `u6s2n-gx777-77774-qaaba-cai`; this is Candid's canister that is necessary in order to render our canister's functionality.

**Note:** ICP's canister IDs are _non-deterministic_, so you may need to replace the above `uxrrr-q7777-77774-qaaaq-cai` canister ID with the ID that is generated from the [Create and Deploy canister](#3-create-and-deploy-canister) step if you are not wiping the subnet state for consecutive deployments.

![Candid UI interface showing canister methods](https://github.com/user-attachments/assets/34499959-d703-4f54-85f6-f3523e492ae4)

### Canister Interaction

The canister is designed to work with Bitcoin mainnet data through the Maestro API.

**Note:** Regtest support would require modifications to the API endpoints or the use of a regtest-compatible indexing service.

## Available Methods

The canister exposes the following public methods:

### 1. get_address_inscriptions

Retrieves all inscriptions associated with a Bitcoin address.

**Method**: `get_address_inscriptions(address: text, count: text) -> (Result)`

**Parameters**:

-   `address`: Bitcoin address (e.g., "bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl")
-   `count`: Maximum number of inscriptions to return (e.g., "10")

**Returns**: `AddressInscriptions` containing:

-   `data`: Array of inscription details
-   `last_updated`: Block information when data was last updated
-   `next_cursor`: Pagination cursor for additional results

**Authorization**: Only authorized principals can call this method.

**Example Usage**:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'
```

[API Docs: Inscription Info](https://docs.gomaestro.org/bitcoin/blockchain-indexer-api/inscriptions/inscription-info)

### 2. get_utxo_inscriptions

Retrieves inscriptions for a specific UTXO (transaction output).

**Method**: `get_utxo_inscriptions(tx_hash: text, output_index: text) -> (Result_1)`

**Parameters**:

-   `tx_hash`: Transaction hash
-   `output_index`: Output index within the transaction

**Returns**: `UtxoInscriptions` containing:

-   `data`: Array of inscription details for the UTXO
-   `last_updated`: Block information
-   `next_cursor`: Pagination cursor

**Authorization**: Only authorized principals can call this method.

**Example Usage**:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_utxo_inscriptions '("604abd1c0ff2ce5a89b004a0601a75280ed3b76384af37b0a46a23471e9288e7", "1")'
```

[API Docs: Transaction Output Info](https://docs.gomaestro.org/bitcoin/blockchain-indexer-api/transactions/transaction-output-info)

### 3. set_api_key

Sets the Maestro API key for the canister (admin only).

**Method**: `set_api_key(key: text) -> (Result_2)`

**Parameters**:

-   `key`: Maestro API key string

**Authorization**: Only authorized principals can call this method.

**Example Usage**:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev set_api_key '("maestro_api_key")'
```

### 4. get_api_key

Retrieves the current API key (admin only, query method).

**Method**: `get_api_key() -> (text)`

**Returns**: Current API key string

**Authorization**: Only authorized principals can call this method.

**Example Usage**:

```bash
dfx canister call bitcoin-metaprotocols-canister-dev get_api_key '()'
```

## Data Structures

### AddressInscription

```candid
type AddressInscription = record {
    floor_price : int64;
    satoshis : text;
    utxo_block_height : int64;
    utxo_txid : text;
    utxo_vout : int32;
    utxo_sat_offset : int64;
    inscription_id : text;
    collection_symbol : opt text;
    utxo_confirmations : int64;
    omb_color : opt text;
    omb_floor_price : opt int64
};
```

### UtxoInscription

```candid
type UtxoInscription = record {
    inscription_id : text;
    collection_symbol : opt text;
    omb_color : opt text;
    omb_floor_price : opt int64
};
```

### LastUpdated

```candid
type LastUpdated = record {
    block_hash : text;
    block_height : int64
};
```

## Troubleshooting

### Common Issues

1. **"Unauthorized" Error**

    - Ensure your principal is in the authorized callers list
    - Use `dfx identity get-principal` to check your principal ID

2. **API Key Issues**

    - Verify the API key is set correctly
    - Ensure you have a valid Maestro API key
    - Check that the API key has sufficient permissions

3. **Deployment Issues**
    - Ensure all prerequisites are installed
    - Run `make generate_did` before deployment
    - Check that the WebAssembly target is installed

### Getting Canister Information

```bash
# Get canister ID
dfx canister id bitcoin-metaprotocols-canister-dev

# Get canister info
dfx canister info bitcoin-metaprotocols-canister-dev

# Check canister status
dfx canister status bitcoin-metaprotocols-canister-dev
```

### Cycle Management

The canister uses cycles for HTTP outcalls to the Maestro API. Monitor cycle usage:

```bash
# Check cycles balance
dfx canister status bitcoin-metaprotocols-canister-dev

# Add cycles if needed
dfx canister deposit-cycles 1000000000000 bitcoin-metaprotocols-canister-dev
```

## API Rate Limits and Costs

-   Each HTTP request to Maestro API consumes cycles (approximately 1B cycles per request)
-   The canister makes multiple API calls per inscription to fetch complete data:
    -   Address inscriptions API call
    -   Inscription info API call (per inscription)
    -   Collection stats API call (per collection)
    -   OMB color group API call (per inscription)
-   Plan cycle usage accordingly based on expected query volume

### Debugging

Enable debug logging in the canister by checking the `ic_cdk::println!` statements in the source code. These will output to the replica logs during development.

-   [Cycleops](https://cycleops.dev/) for monitoring and topping up canisters.

## 🎉 You’re Done!

You have now walked through a guide on how to deploy and interact with the Maestro Bitcoin Metaprotocols Canister.

Be sure to check out [Maestro's additional services](https://www.gomaestro.org/chains/bitcoin) for further assisting your development of building on Bitcoin.

---

### Support

If you are experiencing any trouble with the above, [submit an issue](https://github.com/maestro-org/maestro-bitcoin-metaprotocols-canister/issues/new) or reach out on <a href="https://discord.gg/ES2rDhBJt3" target="_blank">Discord</a>.
