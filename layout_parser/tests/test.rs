// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use layout_parser::Layout;

const TEST_LAYOUT: &str = r#"
{
  "layout_loc":
    "File \"src/lib/mina_networking/mina_networking.ml\", line 321, characters 8-143:",
  "version_opt": null,
  "type_decl":
    "type response = (State_hash.Stable.V1.t * State_body_hash.Stable.V1.t list) option",
  "bin_io_derived": true,
  "bin_prot_rule": [
    "Option",
    [
      "Tuple",
      [
        [
          "Reference",
          [
            "Resolved",
            {
              "source_type_decl": "type t = { version: int ; t: typ }",
              "ref_rule": [
                "Record",
                [
                  { "field_name": "version", "field_rule": [ "Int" ] },
                  {
                    "field_name": "t",
                    "field_rule": [
                      "Reference",
                      [
                        "Resolved",
                        {
                          "source_type_decl": "type typ = t",
                          "ref_rule": [
                            "Reference",
                            [
                              "Resolved",
                              {
                                "source_type_decl": "type t = Field.t",
                                "ref_rule": [
                                  "Reference",
                                  [
                                    "Resolved",
                                    {
                                      "source_type_decl": "Tick.Field.t",
                                      "ref_rule": [ "String" ]
                                    }
                                  ]
                                ]
                              }
                            ]
                          ]
                        }
                      ]
                    ]
                  }
                ]
              ]
            }
          ]
        ],
        [
          "List",
          [
            "Reference",
            [
              "Resolved",
              {
                "source_type_decl": "type t = { version: int ; t: typ }",
                "ref_rule": [
                  "Record",
                  [
                    { "field_name": "version", "field_rule": [ "Int" ] },
                    {
                      "field_name": "t",
                      "field_rule": [
                        "Reference",
                        [
                          "Resolved",
                          {
                            "source_type_decl": "type typ = t",
                            "ref_rule": [
                              "Reference",
                              [
                                "Resolved",
                                {
                                  "source_type_decl": "type t = Field.t",
                                  "ref_rule": [
                                    "Reference",
                                    [
                                      "Resolved",
                                      {
                                        "source_type_decl": "Tick.Field.t",
                                        "ref_rule": [ "String" ]
                                      }
                                    ]
                                  ]
                                }
                              ]
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                ]
              }
            ]
          ]
        ]
      ]
    ]
  ]
}
"#;

#[test]
fn deserialize_test_layout() {
    let layout: Layout = serde_json::from_str(TEST_LAYOUT).unwrap();
    assert_eq!(
        layout.type_decl,
        "type response = (State_hash.Stable.V1.t * State_body_hash.Stable.V1.t list) option"
    )
}
