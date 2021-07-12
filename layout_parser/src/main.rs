use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod lib;

use crate::lib::Layout;

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Layout, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Layout`.
    let u = serde_json::from_reader(reader)?;

    // Return the `Layout`.
    Ok(u)
}

fn main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
