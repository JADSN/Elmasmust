use std::str::FromStr;

use crate::action::Action;

#[derive(Debug, Default)]
pub struct Sha1(String);

impl FromStr for Sha1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Action for Sha1 {
    fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        use sha1::{Digest, Sha1};

        // * Create a Sha1 object
        let mut hasher = Sha1::new();

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
