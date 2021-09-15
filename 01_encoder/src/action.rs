use std::str::FromStr;

pub trait Action: FromStr {
    fn run(&self) -> Result<String, Box<dyn std::error::Error>>;
}
