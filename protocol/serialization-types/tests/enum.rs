// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_serialization_types_macros::AutoFrom;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::json;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn mina_list_tagged_enum() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        enum EnumA {
            V1,
            V2(i32),
            V3(i32, i32),
        }

        #[derive(Debug, Clone, PartialEq, AutoFrom)]
        #[auto_from(EnumA)]
        enum EnumAListTagged {
            V1,
            V2(i32),
            V3(i32, i32),
        }

        let v1 = EnumAListTagged::V1;
        let v1_str = serde_json::to_string(&v1).unwrap();
        let v1_from_str: EnumAListTagged = serde_json::from_str(&v1_str).unwrap();
        assert_eq!(v1, v1_from_str);

        let v2 = EnumAListTagged::V2(1);
        let v2_str = serde_json::to_string(&v2).unwrap();
        let v2_from_str: EnumAListTagged = serde_json::from_str(&v2_str).unwrap();
        assert_eq!(v2, v2_from_str);

        let v3 = EnumAListTagged::V3(2, 3);
        let v3_str = serde_json::to_string(&v3).unwrap();
        let v3_from_str: EnumAListTagged = serde_json::from_str(&v3_str).unwrap();
        assert_eq!(v3, v3_from_str);

        impl Serialize for EnumAListTagged {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let e: EnumA = self.clone().into();
                let v = serde_json::to_value(e).map_err(<S::Error as serde::ser::Error>::custom)?;
                if v.is_string() {
                    let list_tagged_array = json!([v]);
                    return serializer.serialize_some(&list_tagged_array);
                } else if let Some(m) = v.as_object() {
                    if m.len() != 1 {
                        panic!("Bad enum: {:?}", self);
                    }
                    for (k, v) in m {
                        let list_tagged_array = json!([k, v]);
                        return serializer.serialize_some(&list_tagged_array);
                    }
                }
                serializer.serialize_some(&v)
            }
        }

        impl<'de> Deserialize<'de> for EnumAListTagged {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v = serde_json::Value::deserialize(deserializer)?;
                if let Some(array) = v.as_array() {
                    let e: EnumA = match array.len() {
                        0 => panic!("Non-empty array expected"),
                        1 => serde_json::from_value(array[0].clone())
                            .map_err(<D::Error as serde::de::Error>::custom)?,
                        2 => {
                            let key: String = serde_json::from_value(array[0].clone())
                                .map_err(<D::Error as serde::de::Error>::custom)?;
                            serde_json::from_value(json!({key: array[1]}))
                                .map_err(<D::Error as serde::de::Error>::custom)?
                        }
                        _ => {
                            let key: String = serde_json::from_value(array[0].clone())
                                .map_err(<D::Error as serde::de::Error>::custom)?;
                            serde_json::from_value(json!({key: array[1..]}))
                                .map_err(<D::Error as serde::de::Error>::custom)?
                        }
                    };
                    Ok(e.into())
                } else {
                    panic!("Array expected")
                }
            }
        }
    }
}
