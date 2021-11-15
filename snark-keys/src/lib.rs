
use std::io::{Read, BufRead, BufReader};

mod header;
use header::{KeyFileHeader, FILE_ID};

pub fn read_snark_key_file<R: Read>(r: R) -> KeyFileHeader  {
	let r = BufReader::new(r);
	read_file_id(&mut r)?;
	let header = read_header(&mut r)?;

}

fn read_file_id<R: BufRead>(r: &mut R) -> std::io::Result<()> {
	r.lines().next(); // TODO, actually check it is correct
	Ok(())
}

fn read_header<R: BufRead>(r: &mut R) -> serde_json::Result<KeyFileHeader> {
	serde_json::from_reader(r)
}

fn read_body<R: Read>(r: &mut R) -> () {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn name() {
		unimplemented!();
	}
}
