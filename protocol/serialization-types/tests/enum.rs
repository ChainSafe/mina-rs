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
        }

        #[derive(Debug, Clone, PartialEq, AutoFrom)]
        #[auto_from(EnumA)]
        enum EnumAListTagged {
            V1,
            V2(i32),
            V3(i32, i32),
            V4 { f1: i32 },
            V5 { f1: i32, f2: i32 },
        }

        impl_mina_enum_json_serde!(EnumAListTagged, EnumA);

        let v1 = EnumAListTagged::V1;
        let v1_str = serde_json::to_string(&v1)?;
        let v1_from_str: EnumAListTagged = serde_json::from_str(&v1_str)?;
        assert_eq!(v1, v1_from_str);

        let v2 = EnumAListTagged::V2(1);
        let v2_str = serde_json::to_string(&v2)?;
        let v2_from_str: EnumAListTagged = serde_json::from_str(&v2_str)?;
        assert_eq!(v2, v2_from_str);

        let v3 = EnumAListTagged::V3(2, 3);
        let v3_str = serde_json::to_string(&v3)?;
        let v3_from_str: EnumAListTagged = serde_json::from_str(&v3_str)?;
        assert_eq!(v3, v3_from_str);

        let v4 = EnumAListTagged::V4 { f1: 4 };
        let v4_str = serde_json::to_string(&v4)?;
        let v4_from_str: EnumAListTagged = serde_json::from_str(&v4_str)?;
        assert_eq!(v4, v4_from_str);

        let v5 = EnumAListTagged::V5 { f1: 5, f2: 6 };
        let v5_str = serde_json::to_string(&v5)?;
        let v5_from_str: EnumAListTagged = serde_json::from_str(&v5_str)?;
        assert_eq!(v5, v5_from_str);

        Ok(())
    }
}
