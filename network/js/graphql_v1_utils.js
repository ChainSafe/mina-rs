// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

if (!globalThis.fetch) {
    globalThis.fetch = require('node-fetch')
}

async function fetch_block_json_inner(height, stateHash, network) {
    const url = `https://storage.googleapis.com/mina_network_block_data/${network}-${height}-${stateHash}.json`
    const response = await fetch(url)
    return await response.json()
}

async function fetch_block_json(height, stateHash, network = 'mainnet') {
    let exc
    for (var nRetry = 0; nRetry < 3; nRetry += 1) {
        try {
            return await fetch_block_json_inner(height, stateHash, network)
        } catch (e) {
            exc = e
        }
    }
    throw exc
}

async function fetch_block_json_str(height, stateHash, network = 'mainnet') {
    const json = await fetch_block_json(height, stateHash, network)
    return JSON.stringify(json)
}

module.exports = {
    fetch_block_json, fetch_block_json_str
}