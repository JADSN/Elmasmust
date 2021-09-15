use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd", content = "data", rename_all = "lowercase")]
pub enum Cmd {
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Nil,
}
