# Mina-rs

[<img alt="Apache License" src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge" height="20">](https://opensource.org/licenses/Apache-2.0)
[<img alt="Discord" src="https://img.shields.io/discord/593655374469660673.svg?style=for-the-badge&label=Discord&logo=discord" height="20">](https://discord.gg/Vx4uHpNM)

[![Continuous integration](https://github.com/ChainSafe/mina-rs/actions/workflows/build-and-test.yml/badge.svg?branch=main)](https://github.com/ChainSafe/mina-rs/actions/workflows/build-and-test.yml)
[![codecov](https://codecov.io/gh/ChainSafe/mina-rs/branch/main/graph/badge.svg?token=7YXISNRW48)](https://codecov.io/gh/ChainSafe/mina-rs)
[![dependency status](https://deps.rs/repo/github/ChainSafe/mina-rs/status.svg?style=flat-square)](https://deps.rs/repo/github/ChainSafe/mina-rs)
[![loc](https://tokei.rs/b1/github/ChainSafe/mina-rs?category=code)](https://github.com/ChainSafe/mina-rs)

An implementation of Mina protocol in Rust, with focus on web and Wasm compatibility

** As you can probably tell this is a WIP! Don't use for anything yet **

Rust doc of `main` branch can be found [here](https://chainsafe.github.io/mina-rs/rustdoc/)

## Building

Mina builds with the latest stable version of Rust. See [installation instructions for your OS](https://www.rust-lang.org/tools/install).

There isn't yet a binary to build. All of the crates can be built by running

```shell
cargo build
```

from the project root

## Running Tests

All crate unit tests can be run by running

```shell
cargo test
```

### Serialization tests

Serialization tests only can be run by

```shell
cargo test -p test-serialization
```

It is also possible to run the serialization tests in a Wasm environment using wasm-pack. First install wasm-pack with

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

cd to the serialization tests crate directory

```shell
cd protocol/test-serialization
```

and then run tests with either

```shell
wasm-pack test --node --release
```

or

```shell
wasm-pack test --headless --chrome --release --features browser
```

## ChainSafe Security Policy

### Reporting a Security Bug

We take all security issues seriously, if you believe you have found a security issue within a ChainSafe
project please notify us immediately. If an issue is confirmed, we will take all necessary precautions
to ensure a statement and patch release is made in a timely manner.

Please email us a description of the flaw and any related information (e.g. reproduction steps, version) to
[security at chainsafe dot io](mailto:security@chainsafe.io).

## License

Mina-rs is licensed under [Apache 2.0](https://github.com/ChainSafe/mina-rs/blob/main/LICENSE).
