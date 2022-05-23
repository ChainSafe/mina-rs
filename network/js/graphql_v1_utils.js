// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

if (!globalThis.fetch) {
    globalThis.fetch = require('node-fetch')
}

async function retry(func, nRetry = 3) {
    let exc
    for (var nRetry = 0; nRetry < 3; nRetry += 1) {
        try {
            return await func()
        } catch (e) {
            exc = e
        }
    }
    throw exc
}

async function fetch_block_json_inner(height, stateHash, network) {
    const url = `https://storage.googleapis.com/mina_network_block_data/${network}-${height}-${stateHash}.json`
    const response = await fetch(url)
    return await response.json()
}

async function fetch_block_json(height, stateHash, network = 'mainnet') {
    return retry(async function () {
        return await fetch_block_json_inner(height, stateHash, network)
    });
}

async function fetch_block_json_str(height, stateHash, network = 'mainnet') {
    const json = await fetch_block_json(height, stateHash, network)
    return JSON.stringify(json)
}

async function query_latest_blocks_json_inner(limit) {
    const query = `
      query Query {
        blocks(limit: ${limit}, sortBy: RECEIVEDTIME_DESC) {
          blockHeight
          stateHash
        }
      }      
    `
    const url = `https://graphql.minaexplorer.com/`
    const response = await fetch(url, {
        method: 'POST',
        body: JSON.stringify({
            operationName: 'Query',
            query,
        }),
        headers: {
            'Content-Type': 'application/json'
        },
    })
    const json = await response.json()
    return json.data.blocks
}

async function query_latest_blocks_json(limit = 10) {
    return retry(async function () { return await query_latest_blocks_json_inner(limit) })
}

async function query_latest_blocks_json_str(limit = 10) {
    const json = await query_latest_blocks_json(limit)
    return JSON.stringify(json)
}

module.exports = {
    fetch_block_json, fetch_block_json_str, query_latest_blocks_json, query_latest_blocks_json_str
}
