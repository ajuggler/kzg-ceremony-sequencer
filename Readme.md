# KZG Ceremony Rest API

![lines of code](https://img.shields.io/tokei/lines/github/ethereum/kzg-ceremony-sequencer)
[![dependency status](https://deps.rs/repo/github/ethereum/kzg-ceremony-sequencer/status.svg)](https://deps.rs/repo/github/ethereum/kzg-ceremony-sequencer)
[![codecov](https://codecov.io/gh/ethereum/kzg-ceremony-sequencer/branch/main/graph/badge.svg?token=WBPZ9U4TTO)](https://codecov.io/gh/ethereum/kzg-ceremony-sequencer)
[![CI](https://github.com/ethereum/kzg-ceremony-sequencer/actions/workflows/build-test-deploy.yml/badge.svg)](https://github.com/ethereum/kzg-ceremony-sequencer/actions/workflows/build-test-deploy.yml)

This implements [KZG Ceremony Specification](https://github.com/ethereum/kzg-ceremony-specs).

The latest build is available as a container on [ethereum/kzg-ceremony-sequencer](https://hub.docker.com/repository/docker/ethereum/kzg-ceremony-sequencer/general):

```shell
docker run ethereum/kzg-ceremony-sequencer:latest
```

## Setup

### Build, lint, test, run

```shell
cargo fmt && cargo clippy --workspace --all-targets --all-features && cargo build --workspace --all-targets --all-features && cargo test --workspace --all-targets --all-features && cargo run -- -vvv
```

## Requirements

- OAuth Client App: Users sign in with GitHub, which requires an OAuth client application that the user gives read access to their profile to.

## Live URL

- <https://kzg-ceremony-sequencer-dev.fly.dev/info/status>

## Registering for GitHub OAuth

Register for Github OAuth access [here](https://github.com/settings/developers).


```shell
fly secrets set ETH_RPC_URL="..."
fly secrets set ETH_CLIENT_ID="..."
fly secrets set ETH_CLIENT_SECRET="..."
fly secrets set GH_CLIENT_ID="..."
fly secrets set GH_CLIENT_SECRET="..."
fly volumes create kzg_ceremony_sequencer_dev_data --size 5
```

* Fly server: <https://kzg-ceremony-sequencer-dev.fly.dev/info/status>
* Fly dashboard: <https://fly.io/apps/kzg-ceremony-sequencer-dev>
