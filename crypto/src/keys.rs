
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PublicKey([u8; 32]);
