
# bin-prot-checker

A handy cli tool to test [bin prot](https://github.com/janestreet/bin_prot) serialization and deserialization from the command line.

## Build

```bash
cargo build
```

## Ocaml version of bin-prot-checker

Ocaml version of bin-prot-checker resides at: https://github.com/MinaProtocol/mina/tree/bin-prot-checker.
The interop test below relies on the ocaml bin prot checker to be in the root of this repository.

Build instructions:
```
TODO

```

## Running the interop test

The `tests` directory contains the `binprot_interop_test.sh` script that can
be used assert against the two implementations of bin prot spec for making sure
messages are encoded decoded correctly from both sides.

```
cd bin_prot_checker (this repository)
./tests/binprot_interop_test.sh
```
