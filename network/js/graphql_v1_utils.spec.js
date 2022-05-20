// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const { fetch_block_json } = require('./graphql_v1_utils.js')

test("fetch block", async () => {
    let json = await fetch_block_json(25718, '3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn')
    expect(json.protocol_state.body.genesis_state_hash).toBe('3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ');
});
