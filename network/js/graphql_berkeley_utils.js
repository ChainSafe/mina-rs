// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

if (!globalThis.fetch) {
	globalThis.fetch = require("node-fetch");
}

async function retry(func, nRetry = 3) {
	let exc;
	for (var nRetry = 0; nRetry < 3; nRetry += 1) {
		try {
			return await func();
		} catch (e) {
			exc = e;
		}
	}
	throw exc;
}

async function fetch_best_chain_json_inner(endpoint, trackingAccounts) {
	let accountQuery = "";
	if (trackingAccounts) {
		for (var i = 0; i < trackingAccounts.length; i++) {
			const account = trackingAccounts[i];
			// TODO: Query account fields and replace precalculated leafHash after account hashing is fixed
			accountQuery = `
            ${accountQuery}
            
            ${account}: account(publicKey: ${account}) {
                merklePath {
                  left
                  right
                }
                leafHash
              }
            `;
		}
	}
	const query = `
    query Query {
        bestChain(maxLength: 1000000) {
          protocolState {
            previousStateHash
            consensusState {
              blockCreator
              blockHeight
              blockStakeWinner
              epoch
              blockchainLength
              coinbaseReceiever
              epochCount
              hasAncestorInSameCheckpointWindow
              lastVrfOutput
              minWindowDensity
              slot
              slotSinceGenesis
              superchargedCoinbase
              totalCurrency
              nextEpochData {
                epochLength
                lockCheckpoint
                seed
                startCheckpoint
                ledger {
                  hash
                  totalCurrency
                }
              }
              stakingEpochData {
                epochLength
                lockCheckpoint
                seed
                startCheckpoint
                ledger {
                  hash
                  totalCurrency
                }
              }
            }
            blockchainState {
              bodyReference
              date
              snarkedLedgerHash
              stagedLedgerAuxHash
              stagedLedgerHash
              stagedLedgerPendingCoinbaseAux
              stagedLedgerPendingCoinbaseHash
              stagedLedgerProofEmitted
              utcDate
            }
          }
        }
        genesisBlock {
          protocolState {
            blockchainState {
              snarkedLedgerHash
            }
          }
        }
        ${accountQuery}
      }
    `;
	const response = await fetch(endpoint, {
		method: "POST",
		body: JSON.stringify({
			operationName: "Query",
			query,
		}),
		headers: {
			"Content-Type": "application/json",
		},
	});
	const json = await response.json();
	// Patch genesisLedgerHash field in blockChainState
	const data = json.data;
	data.bestChain.protocolState.blockchainState.genesisLedgerHash =
		data.genesisBlock.protocolState.blockchainState.snarkedLedgerHash;
	return data;
}

async function fetch_best_chain_json(endpoint, trackingAccounts) {
	return retry(async function () {
		return await fetch_best_chain_json_inner(endpoint, trackingAccounts);
	});
}

async function fetch_best_chain_json_str(endpoint, trackingAccounts) {
	const json = await fetch_best_chain_json(endpoint, trackingAccounts);
	return JSON.stringify(json);
}

module.exports = {
	fetch_best_chain_json,
	fetch_best_chain_json_str,
};
