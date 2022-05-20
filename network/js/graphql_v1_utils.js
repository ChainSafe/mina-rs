// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

if (!globalThis.fetch) {
    globalThis.fetch = require('node-fetch')
}

export async function fetch_block_json(height, stateHash, network = 'mainnet') {
    const url = `https://storage.googleapis.com/mina_network_block_data/${network}-${height}-${stateHash}.json`
    const response = await fetch(url)
    return await response.json()
}
