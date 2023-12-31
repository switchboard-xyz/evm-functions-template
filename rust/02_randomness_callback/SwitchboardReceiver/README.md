# Switchboard Receiver

<div align="center">
  <img src="https://github.com/switchboard-xyz/sbv2-core/raw/main/website/static/img/icons/switchboard/avatar.png" />

  <h1>Switchboard<br>EVM Functions Template</h1>

  <p>
    <a href="https://discord.gg/switchboardxyz">
      <img alt="Discord" src="https://img.shields.io/discord/841525135311634443?color=blueviolet&logo=discord&logoColor=white" />
    </a>
    <a href="https://twitter.com/switchboardxyz">
      <img alt="Twitter" src="https://img.shields.io/twitter/follow/switchboardxyz?label=Follow+Switchboard" />
    </a>
  </p>
</div>

## Table of Content

- [Prerequisites](#prerequisites)
  - [Installing Docker](#installing-docker)
  - [Docker Setup](#docker-setup)
  - [Build and Push](#build-and-push)
- [Components](#components)
  - [Contract](#contract)
  - [Switchboard Function](#switchboard-function)
  - [Publishing and Initialization](#publishing-and-initialization)
  - [Adding Funding to Function](#adding-funding-to-function)
  - [Printing Function Data](#printing-function-data)
- [Writing Switchboard Rust Functions](#writing-switchboard-rust-functions)
  - [Setup](#setup)
  - [Minimal Example](#minimal-switchboard-function)
  - [Testing your function](#testing-your-function)
  - [Deploying and maintenance](#deploying-and-maintenance)
- [Writing Receiver Contracts](#writing-receiver-contracts)
  - [Receiver Example](#receiver-example)

## Prerequisites

Before you can build and run the project, you'll need to have Docker installed on your system. Docker allows you to package and distribute applications as lightweight containers, making it easy to manage dependencies and ensure consistent behavior across different environments. Switchboard Functions are built and run within containers, so you'll need a docker daemon running to publish a new function.

### Installing Docker

If you don't have Docker installed, you can follow these steps to get it up and running:

1. **Linux**: Depending on your Linux distribution, you might need to use different package managers. For Ubuntu, you can use `apt`:

   ```bash
   sudo apt update
   sudo apt install docker.io
   ```

   For other distributions, consult your package manager's documentation.

2. **macOS**: You can install Docker Desktop for macOS by downloading the installer from the [Docker website](https://www.docker.com/products/docker-desktop) and following the installation instructions.

3. **Windows**: Similarly, you can install Docker Desktop for Windows from the [Docker website](https://www.docker.com/products/docker-desktop) and follow the provided instructions.

### Docker Setup

After installing Docker, make sure it's running by opening a terminal/command prompt and running:

```bash
docker --version
```

This should display the installed Docker version, confirming that Docker is installed and running properly.

You'll need to login to docker. If you don't yet have an account, you'll need one to publish images to dockerhub. You can sign up at [https://hub.docker.com](https://hub.docker.com).

```bash
docker login --username <your-username> --password <your-password>
```

## Components

### Contract

This SwitchboardReceiver contract is a minimal example of a contract producing randomness in a callback function at a scheduled interval.

When you deploy this contract, it will await to be bound to a switchboard function calling into it.

#### Picking a network and setting up your environment

- navigate to the [Project README.md](../../../README.md) and find the switchboard deployment address
- set the `SWITCHBOARD_ADDRESS` env variable to target whichever address is appropriate for the network you're targetting

First, edit the Switchboard import used in your contract in [SwitchboardReceiver](./contracts/src/SwitchboardReceiver.sol) to target the switchboard address for the chain/network you're using.

```solidity
// Core Testnet
import { Switchboard } from "@switchboard-xyz/evm.js/contracts/core/testnet/Switchboard.sol";

// Core Mainnet
import { Switchboard } from "@switchboard-xyz/evm.js/contracts/core/Switchboard.sol";
// or import { Switchboard } from "@switchboard-xyz/evm.js/contracts/core/mainnet/Switchboard.sol";

// Arbitrum Testnet
import { Switchboard } from "@switchboard-xyz/evm.js/contracts/arbitrum/testnet/Switchboard.sol";

// Arbitrum Mainnet
import { Switchboard } from "@switchboard-xyz/evm.js/contracts/arbitrum/Switchboard.sol";
// or import { Switchboard } from "@switchboard-xyz/evm.js/contracts/arbitrum/mainnet/Switchboard.sol";
```

To first deploy the contract, run:

```bash
# ex:
# pnpm deploy:coredaotestnet
# pnpm deploy:coredaomain
# pnpm deploy:arbitrumtestnet
pnpm deploy:${NETWORK_NAME}
```

More deploy commands are available in [package.json](./package.json) scripts.

You will see the last line of this script output

```bash
export SWITCHBOARD_RECEIVER_ADDRESS=<RECEIVER_ADDRESS>
```

### Switchboard Function

Export the address to your environment and navigate to `./switchboard-function/`

The bulk of the function logic can be found in [./switchboard-function/src/main.rs](switchboard-function/src/main.rs).

Build functions from the `switchboard-function/` directory with

```bash
make build
```

### Publishing and Initialization

You'll also need to pick a container name that your switchboard function will use on dockerhub.

```bash
export CONTAINER_NAME=your_docker_username/switchboard-function
```

Here, set the name of your container to deploy and run `make publish`

After this is published, you are free to make your function account to set the rate of run for the function.

### Initializing the function

You'll need the queue id and switchboard contract address from the [Project README.md](../../README.md) for the network you're targetting.

See `scripts/create_function.ts` to create and deploy the function:

```bash
export QUEUE_ID=0x392a3217624aC36b1EC1Cf95905D49594A4DCF64 # placeholder
export SCHEDULE="30 * * * * *" # 30 seconds
export CONTAINER_NAME=switchboardlabs/test
pnpm exec hardhat run scripts/create_function.ts  --network arbitrumTestnet # or coredaoTestnet
```

### Adding Funding to Function

Add funds to your function by doing the following:

```bash
export FUNCTION_ID=0x96cE076e3Dda35679316b12F2b5F7b4A92C9a294
export ETH_VALUE="0.1"
pnpm exec hardhat run scripts/extend_function.ts  --network arbitrumTestnet
```

### Printing Function Data

Now view your function config to ensure it is to your liking:

```bash
export FUNCTION_ID=0x96cE076e3Dda35679316b12F2b5F7b4A92C9a294
pnpm exec hardhat run scripts/check_function.ts  --network arbitrumTestnet
```

## Writing Switchboard Rust Functions

In order to write a successfully running switchboard function, you'll need to import `switchboard-evm` to use the libraries which communicate the function results (which includes transactions to run) to the Switchboard Verifiers that execute these metatransactions.

### Setup

Cargo.toml

```toml
[package]
name = "function-name"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "function-name"
path = "src/main.rs"

[dependencies]
tokio = "^1"
futures = "0.3"

# at a minimum you'll need to include the following packages
ethers = { version = "2.0.7", features = ["legacy"] } # legacy is only for networks that do not support https://eips.ethereum.org/EIPS/eip-2718
switchboard-evm = "0.3.9"
```

### Minimal Switchboard Function

main.rs

```rust
use ethers::{
    prelude::{abigen, ContractCall, SignerMiddleware},
    providers::{Http, Provider},
    types::U256,
};
use rand;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use switchboard_evm::sdk::{EVMFunctionRunner, EVMMiddleware};

#[tokio::main(worker_threads = 12)]
async fn main() {
    // define the abi for the functions in the contract you'll be calling
    // -- here it's just a function named "callback", expecting a random u256
    abigen!(
        Receiver,
        r#"[
            function callback(uint256)
        ]"#,
    );

    // Generates a new enclave wallet, pulls in relevant environment variables
    let function_runner = EVMFunctionRunner::new().unwrap();

    // set the gas limit and expiration date
    let gas_limit = 1000000;

    let expiration_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
        + 64;

    // create a client, wallet and middleware. This is just so we can create the contract instance and sign the txn.
    // @TODO: update the provider to whichever network you're using
    let provider = Provider::<Http>::try_from("https://rpc.test.btcs.network").unwrap();
    let client = Arc::new(
        SignerMiddleware::new_with_provider_chain(
            provider.clone(),
            function_runner.enclave_wallet.clone(),
        )
        .await
        .unwrap(),
    );

    // @TODO: your target contract address here
    // get contract address from docker env
    let contract_address = env!("SWITCHBOARD_RECEIVER_ADDRESS")
        .parse::<ethers::types::Address>()
        .unwrap();

    let receiver_contract = Receiver::new(contract_address, client);

    // generate a random number U256
    let random: [u64; 4] = rand::random();
    let random = U256(random);

    // call function
    let contract_fn_call: ContractCall<EVMMiddleware<_>, _> = receiver_contract.callback(random);

    // create a vec of contract calls to pass to the function runner
    let calls = vec![contract_fn_call.clone()];

    // Emit the result
    function_runner
        .emit(
            contract_address,
            expiration_time.try_into().unwrap(),
            gas_limit.into(),
            calls,
        )
        .unwrap();
}
```

### Testing your function

We can't guarantee that the function will run on the blockchain, but we can test that it compiles and runs locally.

Run the following to test your function:

```bash
export CHAIN_ID=12345 # can be any integer
export VERIFYING_CONTRACT=$SWITCHBOARD_ADDRESS # can be any valid address
export FUNCTION_KEY=$FUNCTION_ID # can be any valid address
cargo build
cargo run # Note: this will include a warning about a missing quote which can be safely ignored.
```

Successful output:

```bash
WARNING: Error generating quote. Function will not be able to be transmitted correctly.
FN_OUT: 7b2276657273696f6e223a312c2271756f7465223a5b5d2c22666e5f6b6579223a5b3134342c32332c3233322c34342c39382c32302c39372c3232392c3138392c33302c3235322c3133362c37362c332c3136382c3130362c3138322c34352c3137352c3137325d2c227369676e6572223a5b3135382c32332c3137302c3133322c3230302c3130322c35302c38352c31302c3134382c3235322c35372c3132362c372c31372c32352c37322c3131342c38322c3134365d2c22666e5f726571756573745f6b6579223a5b5d2c22666e5f726571756573745f68617368223a5b5d2c22636861696e5f726573756c745f696e666f223a7b2245766d223a7b22747873223a5b7b2265787069726174696f6e5f74696d655f7365636f6e6473223a313639313633383836332c226761735f6c696d6974223a2235353030303030222c2276616c7565223a2230222c22746f223a5b38332c3130372c3135352c35382c39382c3132382c37332c3233392c3134382c3133332c3133342c33392c3131382c31362c34382c3235302c3130372c3133382c3234382c3135375d2c2266726f6d223a5b3135382c32332c3137302c3133322c3230302c3130322c35302c38352c31302c3134382c3235322c35372c3132362c372c31372c32352c37322c3131342c38322c3134365d2c2264617461223a5b3136302c3232332c3131392c3130362...
```

### Deploying and Maintenance

After you publish the function and create it on the blockchain, you must keep the function escrow account funded to cover gas fees. Revisions to the function can be made by deploying a new version and updating the function config on-chain.

## Writing Receiver Contracts

While Switchboard Functions can call back into any number of on-chain functions, it's useful to limit access to some privileged functions to just _your_ Switchboard Function.

In order to do this you'll need to know the switchboard address you're using, and which functionId will be calling into the function in question.

### Receiver Example

Recipient.sol

```sol
//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// EIP2771 Context
// Inherited by all contracts that are recipients of switchboard callbacks
contract Recipient {
  address immutable switchboard;

  constructor(address _switchboard) {
    switchboard = _switchboard;
  }

  // get the encoded sender if this message is coming from the switchboard contract
  // if things are working as intended, the sender will be the functionId

  function getEncodedFunctionId() internal view returns (address payable signer) {
    signer = payable(msg.sender);
    if (msg.data.length >= 20 && signer == switchboard) {
      assembly {
        signer := shr(96, calldataload(sub(calldatasize(), 20)))
      }
    }
  }
}
```

Example.sol

```sol
//SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import { Recipient } from "./Recipient.sol";

contract ReceiverExample is Recipient {
  uint256 public randomValue;
  address functionId;

  event NewRandomValue(uint256 value);

  constructor(
    address _switchboard // Switchboard contract address
  ) Recipient(_switchboard) {}

  function callback(uint256 value) external {
    // extract the sender from the callback, this validates that the switchboard contract called this function
    address msgSender = getEncodedFunctionId();

    if (functionId == address(0)) {
      // set the functionId if it hasn't been set yet
      functionId = msgSender;
    }

    // make sure the encoded caller is our function id
    if (msgSender != functionId) {
      revert("Invalid sender");
    }

    // set the random value
    randomValue = value;

    // emit an event
    emit NewRandomValue(value);
  }
}
```
