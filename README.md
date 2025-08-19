# Bitcoin Metaprotocols Canister

[ICP canister](https://internetcomputer.org/docs/building-apps/essentials/canisters) for Bitcoin metaprotocols. Born out of a collaboration between Maestro and Liquidium.

## Current deployments

-   DEV: https://dashboard.internetcomputer.org/canister/gdne4-jqaaa-aaaap-qp3ya-cai
-   PROD: https://dashboard.internetcomputer.org/canister/iayqr-yaaaa-aaaar-qbopq-cai

## Prerequisites

To get started, ensure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [DFX](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/)
-   WebAssembly target for Rust:

```bash
rustup target add wasm32-unknown-unknown
```

## Running Locally

### Setup local admin wallet

### Start Local Subnet

Start the local subnet in a dedicated terminal:

```bash
dfx start --clean
```

### Generate DID (Optional)

```bash
dfx generate
```

### Canister Management

NOTE: If the `--network` argument is not provided, it defaults to the public [playground](https://internetcomputer.org/docs/building-apps/developing-canisters/custom-networks). For local deployments use `--network=local`.

#### Create Canister

```bash
dfx canister create bitcoin-metaprotocols-canister-dev
```

#### Build Canister (Optional)

```bash
dfx build
```

### Generate Candid (Optional)

Prerequisite:

-   Get the `didc` binary from https://github.com/dfinity/candid/releases.
-   Install ic-wasm: `cargo install ic-wasm`

Generate candid:

```
make generate_did
```

#### Deploy Canister

```bash
make generate_did
dfx deploy
```

#### Get Canister Info

```bash
dfx canister info bitcoin-metaprotocols-canister-dev
```

#### Update Canister Settings

Set the canister principal:

```bash
dfx canister update-settings bitcoin-metaprotocols-canister-dev --set-controller <id>
```

### API Key Management

Set the API key:

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev set_api_key '("<maestro_api_key>")'
```

### Testing

#### Test Address Inscriptions

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'
```

#### Test UTXO Inscriptions

```bash
dfx canister call --update bitcoin-metaprotocols-canister-dev get_utxo_inscriptions '("604abd1c0ff2ce5a89b004a0601a75280ed3b76384af37b0a46a23471e9288e7", "1")'
```

## Other resources

-   [Cycleops](https://cycleops.dev/) for monitoring and topping up canisters.
-   [Bitcoin subnet](https://dashboard.internetcomputer.org/network/subnets/pzp6e-ekpqk-3c5x7-2h6so-njoeq-mt45d-h3h6c-q3mxf-vpeq5-fk5o7-yae)
