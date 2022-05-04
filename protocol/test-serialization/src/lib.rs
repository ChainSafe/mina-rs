// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

mod e2e;
mod fuzz;
mod genesis;
mod json;
#[allow(non_snake_case)]
mod test_3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK;

#[cfg(test)]
mod tests {
    use bin_prot::Value;
    use std::str::FromStr;

    pub(crate) fn select_path<'a>(
        block: &'a bin_prot::Value,
        path: impl AsRef<str>,
    ) -> &'a bin_prot::Value {
        // pull out the bin_prot::Value corresponding to the path
        // will panic if the path is invalid
        let path_ref = path.as_ref();
        if path_ref.len() == 0 {
            return block;
        }
        let mut val = block;
        for p in path_ref.split('/') {
            if p == "[sum]" {
                match val {
                    Value::Sum {
                        ref value, index, ..
                    } => {
                        println!("Unpacking sum type index {index} for {path_ref}");
                        val = value;
                    }
                    _ => assert!(false, "Sum expected"),
                }
            } else {
                val = match usize::from_str(p) {
                    Ok(index) => &val[index],
                    _ => &val[p],
                };
            }
        }
        val
    }
}
