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

## Deployment Guide

### Local Development Deployment

#### 1. Start Local ICP Subnet

Start the local Internet Computer subnet in a dedicated terminal:

```bash
dfx start --clean
```

#### 2. Generate Candid Interface (Optional)

```bash
dfx generate
```

#### 3. Create and Deploy Canister

Create the development canister:

```bash
dfx canister create bitcoin-metaprotocols-canister-dev
```

Generate the Candid interface and build:

```bash
make generate_did
```

Deploy the canister:

```bash
dfx deploy
```

#### 4. Set API Key

The canister requires a Maestro API key to function. Set it using:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev set_api_key '("your_maestro_api_key_here")'
```

### Production Deployment to ICP Mainnet

#### 1. Configure Network

Ensure your `dfx.json` is configured for mainnet deployment. The canister supports both development and production configurations:

-   `bitcoin-metaprotocols-canister-dev` - Development canister
-   `bitcoin-metaprotocols-canister-prod` - Production canister

#### 2. Deploy to Mainnet

```bash
# Build and generate Candid
make generate_did

# Deploy to mainnet
dfx deploy --network ic bitcoin-metaprotocols-canister-prod

# Set the API key for production
dfx canister call --network ic --update bitcoin-metaprotocols-canister-prod set_api_key '("your_maestro_api_key_here")'
```

#### 3. Manage Canister Settings

Update canister controllers if needed:

```bash
dfx canister update-settings bitcoin-metaprotocols-canister-prod --network ic --set-controller <controller_principal_id>
```

### Bitcoin Regtest Setup

The canister is designed to work with Bitcoin mainnet data through the Maestro API. For regtest environments:

1. **API Configuration**: The canister uses `https://xbt-mainnet.gomaestro-api.org/v0` as the base URL
2. **Testing**: For regtest testing, you would need:
    - A local Bitcoin regtest network
    - Modified API endpoints (if available for regtest)
    - Or mock data for testing purposes

**Note**: The current implementation is configured for mainnet Bitcoin data. Regtest support would require modifications to the API endpoints or the use of a regtest-compatible indexing service.

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

**Example Usage**:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'
```

[Inscription Info API Reference](https://docs.gomaestro.org/bitcoin/blockchain-indexer-api/inscriptions/inscription-info)

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

**Example Usage**:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_utxo_inscriptions '("604abd1c0ff2ce5a89b004a0601a75280ed3b76384af37b0a46a23471e9288e7", "1")'
```

[Transaction Output Info API Reference](https://docs.gomaestro.org/bitcoin/blockchain-indexer-api/transactions/transaction-output-info)

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

## Authorization

The canister implements access control through a hardcoded list of authorized principals. Only the following principals can call the canister methods:

-   Maestro principals
-   Liquidium principals
-   Other authorized entities

**Current authorized callers**:

-   `62ick-jmsqq-h6wq5-emdfw-qblno-qphae-hs7y3-dxoyp-xiccq-bw4q3-aae`
-   `xktoe-jjqeb-tzsr3-hxjir-en65h-6agv7-bbq2g-dyoch-276wj-waea7-rqe`
-   `roqha-4aaaa-aaaap-qplnq-cai`
-   `e453p-eqaaa-aaaar-qanya-cai`
-   `vr4ua-siaaa-aaaar-qaosq-cai`
-   `pimqm-2dtug-w3ejt-krqai-jlp3u-uux2y-erjcw-wbvhu-pmvhu-hunju-wqe`
-   `daoh3-exchb-6dvbd-fyxld-7kxjo-fdddf-4vhqp-mcoo2-s7gqh-qwpfd-pae`

## Troubleshooting

### Common Issues

1. **"Unauthorized" Error**

    - Ensure your principal is in the authorized callers list
    - Use `dfx identity get-principal` to check your principal ID

2. **API Key Issues**

    - Verify the API key is set correctly
    - Ensure you have a valid Maestro API key
    - Check that the API key has sufficient permissions

3. **HTTP Request Failures**

    - Check internet connectivity
    - Verify Maestro API service status
    - Ensure sufficient cycles are available for HTTP outcalls

4. **Deployment Issues**
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

## Development

### Building Locally

```bash
# Build the canister
cargo build --target wasm32-unknown-unknown --release

# Generate Candid interface
make generate_did
```

### Debugging

Enable debug logging in the canister by checking the `ic_cdk::println!` statements in the source code. These will output to the replica logs during development.

-   [Cycleops](https://cycleops.dev/) for monitoring and topping up canisters.

## Support and Contributing

For issues, feature requests, or contributions:

-   [Submit an issue](https://github.com/maestro-org/maestro-bitcoin-metaprotocols-canister/issues/new)
-   Ensure you follow the existing code style and patterns
-   Test thoroughly before submitting changes

## License

This project is licensed under the [Apache 2.0 License](../LICENSE).
