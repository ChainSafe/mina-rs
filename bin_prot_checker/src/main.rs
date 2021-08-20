// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::fs::File;
use std::io::Seek;
use std::io::{stdin, stdout, Read};
use std::path::PathBuf;

use bin_prot::{error::Error, from_reader, integers::integer, to_writer};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Subcommand,
    #[structopt(long)]
    test: Test,
    /// Output file, stdout if not present
    #[structopt(long, parse(from_os_str))]
    path: Option<PathBuf>,
}

#[derive(StructOpt)]
enum Subcommand {
    Deserialize,
    Serialize,
}

#[derive(Serialize, Deserialize, Debug)]
enum E {
    A,
    B,
    C,
}

#[derive(Serialize, Deserialize, Debug)]
struct S {
    a: i32,
    b: bool,
    c: E,
}

#[derive(Serialize, Deserialize, Debug)]
enum V {
    A(i32),
    B(bool),
    C(E),
}

#[derive(Serialize)]
#[serde(untagged)] // ensures serializing a test enum just serializes the internal data
enum Test {
    Nat0(Vec<()>),
    Bool(bool),
    Int(i32),
    Int32(#[serde(with = "integer")] i32),
    Int64(#[serde(with = "integer")] i64),
    Enum(E),
    Record(S),
    Variant(V),
}

// from str also provides the default type for each test variant
impl FromStr for Test {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nat0" => {
                Ok(Test::Nat0(vec![()]))
            }
            "bool" => {
                Ok(Test::Bool(true))
            }
            "int" => {
                Ok(Test::Int(12345))
            }
            "int32" => {
                Ok(Test::Int32(12345))
            }
            "int64" => {
                Ok(Test::Int64(12345))
            }
            "enum" => {
                Ok(Test::Enum(E::A))
            }
            "record" => {            
                let v = S{
                    a: 15,
                    b: true,
                    c: E::C
                };
                Ok(Test::Record(v))
            }
            "variant" => {
                let v = V::A(15);
                Ok(Test::Variant(v))
            }
            "public-key" => {
                unimplemented!()
            }
            "all" => {
                unimplemented!()
            }
            _ => panic!("Invalid test, Must be one of nat0, bool, int, int32, int64, enum, record, variant, public-key")
        }
    }
}

impl ToString for Test {
    fn to_string(&self) -> String {
        match self {
            Test::Nat0(v) => v.len().to_string(),
            Test::Bool(v) => v.to_string(),
            Test::Int(v) => v.to_string(),
            Test::Int32(v) => v.to_string(),
            Test::Int64(v) => v.to_string(),
            Test::Enum(v) => format!("{:?}", v),
            Test::Record(v) => format!("{:?}", v),
            Test::Variant(v) => format!("{:?}", v),
        }
    }
}

fn deserialize_test<R: Read>(read: R, test: &Test) -> Result<Test, Error> {
    match test {
        Test::Nat0(_) => Ok(Test::Nat0(from_reader(read)?)),
        Test::Bool(_) => Ok(Test::Bool(from_reader(read)?)),
        Test::Int(_) => Ok(Test::Int(from_reader(read)?)),
        Test::Int32(_) => Ok(Test::Int32(from_reader(read)?)),
        Test::Int64(_) => Ok(Test::Int64(from_reader(read)?)),
        Test::Enum(_) => Ok(Test::Enum(from_reader(read)?)),
        Test::Record(_) => Ok(Test::Record(from_reader(read)?)),
        Test::Variant(_) => Ok(Test::Variant(from_reader(read)?)),
    }
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Subcommand::Serialize => {
            if let Some(path) = opt.path {
                println!("Writing to file {:?}", path);
                let mut file = File::create(path).unwrap();
                to_writer(&mut file, &opt.test).map(|_| {
                    let pos = file.stream_position().unwrap();
                    println!("Wrote {} bytes", pos);
                })
            } else {
                to_writer(&mut stdout(), &opt.test)
            }
            .map_err(|e| eprintln!("Failed with: {}", e))
            .unwrap();
        }
        Subcommand::Deserialize => {
            if let Some(path) = opt.path {
                let file = File::open(path).unwrap();
                deserialize_test(file, &opt.test)
            } else {
                deserialize_test(stdin(), &opt.test)
            }
            .map(|v| println!("Deserialized value: {}", v.to_string()))
            .map_err(|e| eprintln!("Failed with: {}", e))
            .unwrap();
        }
    }
}
