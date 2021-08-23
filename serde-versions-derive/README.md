# Serde Versions Derive

`serde_versions_derive` exports an attribute macro that adds versioning support for structs.
 
When serializing a named field struct it will automatically add a new field containing the version.
It also allows deserializing the versioned type directly back to the unversioned one.

Under the hood this works by creating a new struct that wraps the original struct plus adds a version byte field.

Internally this new struct uses `#[serde(flatten)]` to serialize as expected.
The original struct uses `#[serde(to, from)]` to add the version field when serializing and remove it when deserializing.


## usage: 
```rust
#[version(3)]
#[derive(Clone, Serialize, Deserialize)]
struct S {
    i: i32,
}
```

This produces the following
```rust
#[derive(Clone, Serialize, Deserialize)]
#[serde(into = "_Sv3", from = "_Sv3")]
struct S {
    i: i32,
}

#[derive(Clone, Serialize, Deserialize)]
struct _Sv3 {
    version: u8,
    #[serde(flatten)]
    inner: S
}

// plus implementations of To, From and to_versioned() for S
```

and will Serialize to:

```json
{
	"version": 3,
	"i": 0
}
```


---

This supports types with type parameters however these must have a trait bound
to implement Clone

e.g.:
```rust
# use serde::{Deserialize, Serialize};
# use serde_versions_derive::version;
#[version(3)]
#[derive(Clone, Serialize, Deserialize)]
struct S<T: Clone> {
    t: T,
} 
