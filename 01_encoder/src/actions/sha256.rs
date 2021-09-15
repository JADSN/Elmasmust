use std::str::FromStr;

use crate::action::Action;

#[derive(Debug, Default)]
pub struct Sha256(String);

impl FromStr for Sha256 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Action for Sha256 {
    fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        use sha2::{Digest, Sha256};

        // * Create a Sha256 object
        let mut hasher = Sha256::new();

        // * Write input message
        hasher.update(self.0.as_bytes());

        // * Read hash digest and consume hasher
        let hash = hasher.finalize();
        let mut result = "".to_string();

        for byte in hash {
            let serialized_byte = format!("{:02x}", byte);
            result.push_str(&serialized_byte)
        }

        Ok(result)
    }
}
