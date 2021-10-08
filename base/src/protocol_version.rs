
use serde::{Serialize, Deserialize};
use wire_type::WireType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProtocolVersion {
	major: u32,
	minor: u32,
	patch: u32,
}
