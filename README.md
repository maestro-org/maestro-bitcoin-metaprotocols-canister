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

Follow the below steps or follow our [usage guide](./docs/usage-guide.md) for a more guided walkthrough of deploying and interacting with the canister.

### Setup identity (Optional)

Create new dedicated [identity](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/dfx-identity), or:

Use the default one for local development:

```bash
dfx identity use default
```

If the `--network` argument is not provided, it defaults to the public [playground](https://internetcomputer.org/docs/building-apps/developing-canisters/custom-networks). For local deployments use `--network=local`. For mainnet use `--network=ic`.

### Start Local Subnet

Start the local subnet in a dedicated terminal:

```bash
dfx start --clean
```

### Canister Management

#### Create Canisters

```bash
dfx canister create --network=local bitcoin-metaprotocols-canister-dev
```

### Generate DID

```bash
dfx generate --network=local bitcoin-metaprotocols-canister-dev
```

#### Build Canister

```bash
dfx build --network=local bitcoin-metaprotocols-canister-dev
```

#### Deploy Canister

```bash
dfx deploy --network=local bitcoin-metaprotocols-canister-dev
```

#### Get Canister Info

```bash
dfx canister info --network=local bitcoin-metaprotocols-canister-dev
```

#### Update Canister Settings (Optional)

Set the canister principal:

```bash
dfx canister update-settings --network=local bitcoin-metaprotocols-canister-dev --set-controller <id>
```

### API Key Management

Set the API key:

```bash
dfx canister call --network=local --update bitcoin-metaprotocols-canister-dev set_api_key '("<maestro_api_key>")'
```

### Testing

#### Test Address Inscriptions

```bash
dfx canister call --network=local --update bitcoin-metaprotocols-canister-dev get_address_inscriptions '("bc1pa2lw8d6u3kkexzqn9hqgzultkzjjc9rxtveldes68ryfdq8tmslqwfuccl", "10")'
```

#### Test UTXO Inscriptions

```bash
dfx canister call --network=local --update bitcoin-metaprotocols-canister-dev get_utxo_inscriptions '("604abd1c0ff2ce5a89b004a0601a75280ed3b76384af37b0a46a23471e9288e7", "1")'
```

#### Monitor canister logs

```bash
dfx canister logs --network=local bitcoin-metaprotocols-canister-dev
```

## Other resources

-   [Cycleops](https://cycleops.dev/) for monitoring and topping up canisters.
-   [Bitcoin subnet](https://dashboard.internetcomputer.org/network/subnets/pzp6e-ekpqk-3c5x7-2h6so-njoeq-mt45d-h3h6c-q3mxf-vpeq5-fk5o7-yae)
-   [Networks](https://internetcomputer.org/docs/building-apps/developing-canisters/custom-networks#custom-dfx-networks)
