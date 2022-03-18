use serde::{Deserialize, Serialize};

pub mod memo;
pub mod payment;
pub mod signed_command;

pub use memo::SignedCommandMemo;
pub use payment::PaymentPayload;
pub use signed_command::{
    SignedCommand, SignedCommandPayload, SignedCommandPayloadBody, SignedCommandPayloadCommon,
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum UserCommand {
    SignedCommand(SignedCommand),
    // FIXME: other variants are not covered by current test block
}

impl Default for UserCommand {
    fn default() -> Self {
        Self::SignedCommand(SignedCommand::default())
    }
}
