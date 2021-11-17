use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct VerificationKey {
	// constraint system compilation (Dee.Affine.t array Abc.t Matrix_evals.t)
	// a MatrixEvals of ABC of arrays of Dee.Affine (elliptic curve points)
	commitments: , 
	// evaluation domains as multiplicative groups of roots of unity (Domains.t array)
	// an array of Domains
	step_domains: Vec<Domain>, 
	data: Data,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
struct Domains {
	h: Domain,
	k: Domain,
	x: Domain
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
enum Domain {
	Pow2RootsOfUnity(usize)
}

impl Default for Domain {
	fn default() -> Self {
		Self::Pow2RootsOfUnity(0)
	}
}

/// Data associated with a verification key
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
struct Data {
	public_input: usize,
	variables: usize,
	constraints: usize,
	nonzero_entries: usize,
	max_degree: usize, // max_poly_size in arkworks
}

/// a fancy 3-tuple used in Mina
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
struct ABC {
	a: Vec<FiniteECPoint>,
	b: Vec<FiniteECPoint>,
	c: Vec<FiniteECPoint>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
struct MatrixEvals {
	row: ABC,
	col: ABC,
	value: ABC,
	rc: ABC,
}