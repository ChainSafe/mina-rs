// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum ForkTreeError {
    #[error("Descendant not found")]
    DescendantNotFound,
    #[error("Parent not found")]
    ParentNotFound,
    #[error("Block exists")]
    BlockExists,
    #[error("Start node not found")]
    StartNodeNotFound,
    #[error("End node not found")]
    EndNodeNotFound,
    #[error("Node not found")]
    NodeNotFound,
    #[error("Ancestor not found")]
    AncestorNotFound,
}