use std::str::FromStr;

use crate::{
    action::Action,
    actions::{md5::Md5, sha1::Sha1, sha256::Sha256, sha512::Sha512},
    cmd::Cmd,
};

pub struct Dispatcher;

impl Dispatcher {
    pub fn from_cmd(cmd: &Cmd) -> String {
        match cmd {
            Cmd::Md5(data) => match Md5::from_str(data).unwrap_or_default().run() {
                Ok(data) => data,
                Err(_) => String::from("Oops!"),
            },

            Cmd::Sha1(data) => match Sha1::from_str(data).unwrap_or_default().run() {
                Ok(data) => data,
                Err(_) => String::from("Oops!"),
            },
            Cmd::Sha256(data) => match Sha256::from_str(data).unwrap_or_default().run() {
                Ok(data) => data,
                Err(_) => String::from("Oops!"),
            },
            Cmd::Sha512(data) => match Sha512::from_str(data).unwrap_or_default().run() {
                Ok(data) => data,
                Err(_) => String::from("Oops!"),
            },
            Cmd::Nil => {
                // log!("CAIU NO NILL");

                String::from("asdf")
            }
        }
    }
}
