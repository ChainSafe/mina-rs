# Modifying Serializable Types

Some types in Mina-rs need to be serialized and send over the network to other nodes or stored in disk. For example `ExternalTransition`. Most of the types in the base crate are contained in a type that is serialized at some point.

## Serialization-types crate

Mina-rs must maintain protocol compatibility with other implementations so it is critical that the serialization of these types does not change. To help with this the project contains the `protocol/serialization-types` crate. This crate aims to replicate the structure of the types in Mina-OCaml such that when they are serialized or deserialized using serde, they result in a byte-compatible form. 

!!!
The types in this crate should never be changed unless:
- They are found to be wrong
- There are changes to the wire protocol originating in the OCaml implementation. This will likely require backward compatibility so the types should be extended and not replaced
!!!

## Internal Types

To improve ergonomics, idiomatics and and readability we do not use the serialization types throughout the rest of the codebase. Instead there is an internal representation which must be convertable to and from the serialization type. Some important things to note:

- Only top-level serialization types (those sent over the network) that need to implement this conversion
	- The conversion can be split into multiple simpler conversions to simplify the implementation
- The conversion MUST be lossless. A round-trip conversion must produce an identical result

### Modifying the Internal Types

Internal types can be modified but doing so will break the existing conversion logic. Any changes to internal types that have a serialization type will require making changes to the `serialization_type_conversions` implementation belonging to that type. This should be a relatively simple process for most changes. Existing tests should ensure that these conversions remain loss-less.
