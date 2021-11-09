// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use test_fixtures::TEST_BLOCKS;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "block serialization: 3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK",
        |b| {
            let block = TEST_BLOCKS
                .get("3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex")
                .unwrap();
            let et = block.external_transition().unwrap();
            b.iter(|| {
                let mut output = vec![];
                bin_prot::to_writer(&mut output, black_box(&et)).unwrap();
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
