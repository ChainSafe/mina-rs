
use std::io::{Write, SeekFrom};
use std::fs::File;
use std::path::PathBuf;
use std::io::Seek;

use structopt::StructOpt;

use serde_bin_prot::{to_writer};

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
#[structopt(name = "example", about = "An example of StructOpt usage.")]
enum Subcommand {
    Deserialize ,
    Serialize,
}

fn serialize_test<W: Write>(test: &str, writer: &mut W) -> Result<(), String> {
    match test {
        "nat0" => {
            // will write out the 
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
        // "int32" => {}
        // "int64" => {}
        // "enum" => {}
        // "record" => {}
        // "variant" => {}
        "public-key" => {
            let v = mina_crypto::keys::PublicKey::new();
            to_writer(writer, &v)

        }
        _ => panic!("Invalid test")
    }.map_err(|_| "fucked it".to_string())
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Subcommand::Serialize => {
            let mut f = File::create(opt.path.clone()).expect("Unable to create file");
            serialize_test(&opt.test, &mut f).expect("blerp");
            let bytes_written = f.seek(SeekFrom::Current(0)).unwrap();
            println!("Wrote {} bytes to file {:?}", bytes_written, opt.path)
        },
        Subcommand::Deserialize => {

        }
    }

}
