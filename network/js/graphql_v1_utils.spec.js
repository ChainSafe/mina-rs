// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const { fetch_block_json, fetch_block_json_str, query_latest_blocks_json } = require('./graphql_v1_utils.js')

jest.setTimeout(30000)

test("fetch block json", async () => {
    let json = await fetch_block_json(25718, '3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn')
    expect(json.protocol_state.body.genesis_state_hash).toBe('3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ');
});

test("fetch block json str", async () => {
    let json_str = await fetch_block_json_str(25718, '3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn')
    expect(json_str.length).toBeGreaterThan(0);
});

test("query latest blocks json", async () => {
    const limit = 10;
    let blocks = await query_latest_blocks_json(limit)
    expect(blocks.length).toBe(limit);
    blocks.forEach(block => {
        expect(block.blockHeight).toBeGreaterThan(0)
        expect(block.stateHash.length).toBeGreaterThan(0)
    });
});
