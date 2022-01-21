
// TODO move to own crate once cyclic deps are gone

pub mod external_transition;
pub mod protocol_state;
pub mod protocol_state_body;
pub mod protocol_constants;
pub mod staged_ledger_diff;

/// Version 1 serialization types
pub mod v1 {
	pub use super::external_transition::ExternalTransitionV1;
	pub use super::staged_ledger_diff::StagedLedgerDiffV1;
	pub use super::protocol_state::ProtocolStateV1;
	pub use super::protocol_state_body::ProtocolStateBodyV1;
	pub use super::protocol_constants::ProtocolConstantsV1;
}
