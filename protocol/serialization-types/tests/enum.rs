// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_serialization_types::impl_mina_enum_json_serde;
    use mina_serialization_types_macros::AutoFrom;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn mina_list_tagged_enum_wasm() {
        mina_list_tagged_enum().unwrap();
    }

    #[test]
    fn mina_list_tagged_enum() -> anyhow::Result<()> {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        enum EnumA {
            V1,
            V2(i32),
            V3(i32, i32),
            V4 { f1: i32 },
            V5 { f1: i32, f2: i32 },
            V6(Option<i32>),
        }

        #[derive(Debug, Clone, PartialEq, AutoFrom)]
        #[auto_from(EnumA)]
        enum EnumAListTagged {
            V1,
            V2(i32),
            V3(i32, i32),
            V4 { f1: i32 },
            V5 { f1: i32, f2: i32 },
            V6(Option<i32>),
        }

        impl_mina_enum_json_serde!(EnumAListTagged, EnumA);

        {
            let v = EnumAListTagged::V1;
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V2(1);
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V3(2, 3);
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V4 { f1: 4 };
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V5 { f1: 5, f2: 6 };
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V6(None);
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }
        {
            let v = EnumAListTagged::V6(Some(7));
            let v_str = serde_json::to_string(&v)?;
            let v_from_str: EnumAListTagged = serde_json::from_str(&v_str)?;
            assert_eq!(v, v_from_str);
        }

        Ok(())
    }
}
