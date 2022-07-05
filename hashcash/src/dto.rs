extern crate rand;

use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCash(pub MD5HashCashInput);

impl MD5HashCashInput {
    pub fn new() -> MD5HashCashInput {
        let mut rng = thread_rng();
        let complexity: u32 = rng.gen_range(5..24);
        MD5HashCashInput {
            complexity,
            message: "".to_string(),
        }
    }
}
