
use std::io::{Write, SeekFrom};
use std::fs::File;
use std::path::PathBuf;
use std::io::Seek;

use structopt::StructOpt;
use serde::{Serialize};
use serde_bin_prot::{to_writer, integers::integer};

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Subcommand,
    #[structopt(long, parse(from_os_str))]
    path: PathBuf, 
    #[structopt(long)]
    test: String,
}

#[derive(StructOpt)]
enum Subcommand {
    Deserialize ,
    Serialize,
}

fn serialize_test<W: Write>(test: &str, writer: &mut W) -> Result<(), String> {

    #[derive(Serialize)]
    enum E {A, B, C}

    #[derive(Serialize)]
    struct S {
        a: i32,
        b: bool,
        c: E
    }

    #[derive(Serialize)]
    enum V {
        A(i32),
        B(bool),
        C(E)
    }

    match test {
        "nat0" => {
            // Test nat0 by writing out vectors and ensuring the right length is written
            let v = vec![()];
            to_writer(writer, &v)
        }
        "bool" => {
            let v = true;
            to_writer(writer, &v)
        }
        "int" => {
            let v = 12345;
            to_writer(writer, &v)
        }
        "int32" => {
            #[derive(Serialize)]
            struct I(#[serde(with = "integer")]i32);
            let v = I(12345);
            to_writer(writer, &v)
        }
        "int64" => {
            #[derive(Serialize)]
            struct I(#[serde(with = "integer")]i64);
            let v = I(12345);
            to_writer(writer, &v)
        }
        "enum" => {
            let v = E::A;
            to_writer(writer, &v)
        }
        "record" => {            
            let v = S{
                a: 15,
                b: true,
                c: E::C
            };
            to_writer(writer, &v)            
        }
        "variant" => {
            let v = V::A(15);
            to_writer(writer, &v)
        }
        "public-key" => {
            unimplemented!()
        }
        _ => panic!("Invalid test, Must be one of nat0, bool, int, int32, int64, enum, record, variant, public-key")
    }.map_err(|_| "Unable to write to file".to_string())
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Subcommand::Serialize => {
            let mut f = File::create(opt.path.clone()).expect("Unable to create file");
            if let Err(e) = serialize_test(&opt.test, &mut f) {
                eprintln!("Failed with: {}", e)
            } else {
                let bytes_written = f.seek(SeekFrom::Current(0)).unwrap();
                println!("Wrote {} bytes to file {:?}", bytes_written, opt.path)
            }
        },
        Subcommand::Deserialize => {

        }
    }
}
