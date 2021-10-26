# Mina-rs strong types working bee

This document describes a systematic process by which we can expand the types in Mina-rs in such a way that their serde derived serialization is compatible with wire types from Mina-OCaml.

## Background

Mina-rs uses Serde and the serde bin-prot library to automatically derive the serialization and deserialization code for types. This is a performant and idiomatic approach, however since we must ensure compatibility with an existing protocol the types themselves must be constructed precisely to ensure byte-wise compatibility.

Having acccess to serialized binary of every type would be an ideal place to start however we only have access to serialized binaries of high level types (e.g. `external_transition` AKA `block`). This process along with the loose type and partial type deserialization capabilities of the `bin_prot` library allow for testing the implementation of sub-types in isolation while ensuring their compatibility.

## Process

### 1. Identify the next type to implement

The `strong-types-working-bee` branch should contain the latest additions that have been reviewed and merged. This will be merged to `main` once the process is complete.

Look in the `base` crate for any type that contains a field of type `Value`. The working bee will be complete once all instances of `Value` have been replaced with their correct strong type. It may be useful to start at the `ExternalTransition` type and work your way down.

Also inspect any open PRs with the `type-working-bee` tag to make sure you don't duplicate any work.

#### Example

In `external_transition.rs` we have identified that the `protocol_state` field has an unimplemented type.

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ExternalTransition {
    protocol_state: Value, // <<----------------
    protocol_state_proof: Value,
    staged_ledger_diff: Value,
    delta_transition_chain_proof: Value,

    current_protocol_version: ProtocolVersion,
    proposed_protocol_version_opt: Option<ProtocolVersion>,
    validation_callback: (),
}
```

### 2. Figure out what the new type should look like

There are a few options here. 

#### Option 1 

You can look in the Mina-OCaml repo for the containing type (e.g. [external_transition](https://github.com/MinaProtocol/mina/blob/develop/src/lib/mina_transition/external_transition.ml)) and find the reference to the implementation of the type itself (e.g.[protocol_state](https://github.com/MinaProtocol/mina/blob/develop/src/lib/mina_state/protocol_state.ml)).  

Note that the only thing you need to care about is the type (e.g. Sum->Enum or Record->Struct) and the variant/field names and ordering. You can safely ignore the `Poly`/`Stable`/`V1` stuff. We have our own Rust idioms for dealing with these thing.

##### Example

We identify `ProtocolState` is a struct with 2 fields: `[previous_state_hash, body]`. The order is important!

```ocaml
module Poly = struct
  [%%versioned
  module Stable = struct
    module V1 = struct
      type ('state_hash, 'body) t =
        { previous_state_hash : 'state_hash; body : 'body }
      [@@deriving equal, ord, hash, sexp, yojson, hlist]
    end
  end]
end
```

#### Option 2

You can also look directly at the type [layout file](https://github.com/ChainSafe/mina-rs/blob/main/layouts/external_transition.json). Be warned this is a very big JSON file. It needs to be prettified and then you can read the exact layouts each type is expecting.

When inspecting the layouts you can safely ignore the `version` fields as these are added by the `WireTypes` crate automatically.

I expect this approach will work better for the smaller specialized types (e.g. EllipticCurvePoint) but not well for higher level types (e.g. protocol_state).


### 3. Implement the type

High level types should live in their own files in the `core` crate. Other types (e.g. signatures, snark proofs etc) might be better in the crypto crate or their own crates. Totally up to you.

Implement the new type and use `bin_prot::Value` as the type for any fields that have types which are not implemented. We can work on those later. Be sure to look around and use the correct type for fields that use a type which has already been implemented (e.g. `StateHash`)

#### Example

We will add a new file `protocol_state.rs` with the following. Note the derives and the serde from/into attributes. These are important to ensure it is serialized correctly.

```rust
use wire_type::WireType;
use bin_prot::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[wire_type( recurse = 2 )]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProtocolState {
    previous_state_hash: Value,
    body: Value,
}
```

also modify the containing types (there may be multiple)

```rust
pub struct ExternalTransition {
    protocol_state: ProtocolState, // <<----------------
    protocol_state_proof: Value,
    staged_ledger_diff: Value,
    delta_transition_chain_proof: Value,

    current_protocol_version: ProtocolVersion,
    proposed_protocol_version_opt: Option<ProtocolVersion>,
    validation_callback: (),
}
```

Take careful note of the `#[wire_type( recurse = 2 )]` attribute. This isn't needed in all cases but it is in this case since `ProtocolState` is nested inside two versioned modules in the OCaml code (Poly and Stable).
`

### 4. Add it to the tests

All testing takes place in the `test_serialization` crate.

Just by adding the new type to `ExternalTransition` it will already be tested in `smoke_test_partial_block`. ExternalTransition is the highest level wire type and will eventually contain all other wire types.

---

The other place to add a test is in `test_all_block_subtypes`. This uses the `test_in_block` function to rip out examples of types as indexed by their path in an external_transition and test against them. Types should only be added here if them or their subtypes contain no loose fields. This kind of testing allows bottom-up testing of types if we choose to do that. 

Discovering the correct path can be tricky and may require inspection of the block layout file. There may be a number of paths to the same type so try and find them all.

#### Example

```rust
   #[test]
    fn test_all_block_subtypes() {
        let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, &BLOCK_RULE);
        let block: bin_prot::Value =
            Deserialize::deserialize(&mut de).expect("Failed to deserialize block");

        ////////////////////////////////////////////////////////////////
        // Here is where to add calls to test_in_block for every type
        // that has a strongly typed implementation to test
        ////////////////////////////////////////////////////////////////

        // protocol_version
        test_in_block::<ProtocolVersion>(&block, &["t/current_protocol_version"]);
        test_in_block::<Option<ProtocolVersion>>(&block, &["t/proposed_protocol_version_opt"]);
+       // protocol_state, don't actually do this as it still contains loose fields
+       test_in_block::<ProtocolState>(&block, &["t/protocol_state"]);
    }
```

### Wrap it up

Create a branch with the naming convention `types-working-bee-<type-name>` and make a new PR to the base branch `types-working-bee-base`

### Debugging

#### `Failed to deserialize block: Custom { message: "missing field `<fieldname>`" }``

This happens when the structure of the strong type doesn't match the layout. Some possibilities:

- Forgot to derive `WireType`
- There is nested versioning (e.g. `{"version":1,"t":{"version":1,"t":{"a":123,"b":321}}}`). In this case you can use the `#[wire_types( recurse = 2 )]` attribute
