// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate clap;
extern crate log;
extern crate pretty_env_logger;
extern crate prettytable;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod consts;
pub mod http;
pub mod logger;
pub mod sites;
pub mod wlans;

pub use crate::auth::paths::*;
pub use crate::auth::*;

fn main() {
    pretty_env_logger::init();
    match http::HttpClient::new() {
        Ok(c) => {
            let r = sites::sites::list(&c, "def9f111-57b4-48ee-8474-5edf857abf03");
            println!("{:#?}", r);
        }
        Err(_) => logger::warn("API_TOKEN is not configured"),
    }
}
