# BinProt Layout Reader

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A simple tool that allows reading bin_prot encoded files given a JSON layout describing the shape of the data type


## Usage

The tool can be build and run using 

```shell
cargo run
```

it has two required parameters:


```shell

bin-prot-layout-reader 0.1.0

USAGE:
    bin-prot-layout-reader [OPTIONS] <layout> <binary>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output file, stdout if not present

ARGS:
    <layout>    Input layout JSON file
    <binary>    Input binary file

```

The binary file provide can be either be a file containing a utf-8 encoded hex representation of the encoded bin-io, OR a file to be read directly as the encoded binary itself. The tool will attempt to discover which type has been passed by first attempting to interpret the file contents as utf-8 encoded hex. If this fails it will use the file in its binary form.

## Layouts

For examples of layout files see [the layouts directory](../../protocol/layouts)

Layout files must be deserializable from JSON into the [layout struct](../../protocol/bin-prot/src/value/layout/mod.rs)

## Licence

Distributed under the Apache-2.0 License. See LICENSE for more information.

## Contact

[Willem Olding](mailto:willem@chainsafe.io)
