
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash)]
pub struct TokenId(u64);

