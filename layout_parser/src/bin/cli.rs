use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use layout_parser::Layout;

#[derive(StructOpt)]
struct Opt {
    /// input layout file
    #[structopt(long, parse(from_os_str))]
    path: PathBuf,
}

fn read_layout_from_file<P: AsRef<Path>>(path: P) -> Result<Layout, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Layout`.
    // need to use the disabled recursion limit for large layouts (e.g. external_transition)

    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let u = Layout::deserialize(deserializer)?;

    // Return the `Layout`.
    Ok(u)
}

fn main() {
    let opt = Opt::from_args();
    let u = read_layout_from_file(opt.path).expect("Failed to parse layout");
    println!("{:#?}", u);
}
