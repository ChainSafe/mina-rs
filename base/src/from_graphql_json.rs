// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! GraphQL API json conversion traits and utilities

/// Trait that deserializes a struct from graphql json
pub trait FromGraphQLJson {
    /// Deserialize from graphql json
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self>
    where
        Self: Sized;
}
