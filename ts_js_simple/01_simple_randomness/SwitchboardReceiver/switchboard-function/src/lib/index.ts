import { BigNumber, ethers, utils, Contract } from "ethers";
import { FunctionRunner } from "@switchboard-xyz/evm.js";
import { Functions } from "./Functions";
import { Log } from "./Log";
import { getRequestConfig } from "./utils";
import * as crypto from "crypto";

import * as vm from "node:vm";

// get compiled config
import config from "./config.json";

// Run the switchboard function
async function main() {
  // Create a FunctionRunner
  const runner = new FunctionRunner();

  // Get config for the request
  const requestConfig = getRequestConfig(config);

  // Contract interface for FunctionsClient
  const iface = new ethers.utils.Interface([
    "function callbackBytes(bytes)",
    "function callbackInt256(int256)",
    "function callbackString(string)",
    "function callbackUint256(uint256)",
    "function callbackBytesWithId(bytes,address)",
    "function callbackUint256WithId(uint256,address)",
    "function callbackInt256WithId(int256,address)",
    "function callbackStringWithId(string,address)",
  ]);

  // Get the contract address
  const contract = new Contract(
    process.env.SWITCHBOARD_RECEIVER_ADDRESS,
    iface,
    runner.enclaveWallet
  );

  // Get params from the request
  const requests = runner.rawParams.length
    ? runner.params(["string[]"])
    : [{ callId: "", params: config.args as utils.Result }];

  // get idx of return type
  const returnTypeIdx = ["uint256", "int256", "string", "Buffer"].indexOf(
    config.expectedReturnType
  );

  // match up callback functions with return types
  const callbackFunctions = [
    contract.populateTransaction.callbackUint256,
    contract.populateTransaction.callbackInt256,
    contract.populateTransaction.callbackString,
    contract.populateTransaction.callbackBytes,
  ];

  // match up param-triggered callback functions with return types
  const callbackFunctionsWithId = [
    contract.populateTransaction.callbackUint256WithId,
    contract.populateTransaction.callbackInt256WithId,
    contract.populateTransaction.callbackStringWithId,
    contract.populateTransaction.callbackBytesWithId,
  ];

  // get callback function for return type
  const callback = callbackFunctions[returnTypeIdx];
  const callbackWithId = callbackFunctionsWithId[returnTypeIdx];

  // Handle all params in parallel
  const functionCalls = requests.map(async (request) => {
    // Get params from the request
    const { callId, params: args } = request;

    // Function context
    const context = {
      Functions,
      Log,
      args: args && args.length ? args : [],

      // Add more context utils here
      crypto, // node crypto for randomness
    };

    // Result
    let result: BigNumber | Buffer | string;

    // Try evaluating the javascript function
    // this expects to only run safe/trusted (user-owned) code [equivalent to JS "eval"]
    try {
      result = await vm.runInContext(
        requestConfig.source,
        vm.createContext(context)
      );

      // handle calls with IDs
      if (callId) {
        return await callbackWithId(result, callId);
      } else {
        return await callback(result);
      }
    } catch (err) {
      console.log(err);
      return undefined;
    }
  });

  // wait for all calls to populate, filter bad params
  const calls = (await Promise.all(functionCalls)).filter(
    (c) => c !== undefined
  );

  // emit txn
  await runner.emit(calls);
}

// run switchboard function
main();
