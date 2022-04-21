#! /bin/bash

pushd "$(dirname $0)/../apps"
cargo build --release -p mina-rs-profilers
pushd "target/release"
mkdir profiler-reports || echo 'dir exists'
mkdir profiler-reports/block-serde-profiler || echo 'dir exists'
pushd profiler-reports/block-serde-profiler

./../../block-serde-profiler -m heap
valgrind --tool=callgrind --callgrind-out-file=callgrind.out --  ./../../block-serde-profiler -m cpu
gprof2dot -f callgrind callgrind.out | dot -Tsvg -o callgrind.svg

popd
popd
popd
