// TODO move to own crate once cyclic deps are gone

pub mod external_transition;
pub mod protocol_constants;
pub mod protocol_state;
pub mod protocol_state_body;
pub mod staged_ledger_diff;

/// Version 1 serialization types
pub mod v1 {
    pub use super::external_transition::ExternalTransitionV1;
    pub use super::protocol_constants::ProtocolConstantsV1;
    pub use super::protocol_state::ProtocolStateV1;
    pub use super::protocol_state_body::ProtocolStateBodyV1;
    pub use super::staged_ledger_diff::{
        CoinBaseBalanceDataV1, CoinBaseFeeTransferV1, CoinBaseV1, FeeTransferBalanceDataV1,
        InternalCommandBalanceDataV1, PaymentPayloadV1, SignedCommandFeeTokenV1,
        SignedCommandMemoV1, SignedCommandPayloadBodyV1, SignedCommandPayloadCommonV1,
        SignedCommandV1, StagedLedgerDiffTupleV1, StagedLedgerDiffV1, StagedLedgerPreDiffOneV1,
        StagedLedgerPreDiffTwoV1, TransactionStatusAppliedV1, TransactionStatusAuxiliaryDataV1,
        TransactionStatusBalanceDataV1, TransactionStatusV1, UserCommandV1,
        UserCommandWithStatusV1,
    };
}
