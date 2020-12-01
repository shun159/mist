// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

extern crate clap;
extern crate log;
extern crate pretty_env_logger;
extern crate prettytable;
extern crate serde;
extern crate serde_json;

pub mod auth;
pub mod http;
pub mod logger;
pub mod wlans;

pub use crate::auth::*;
pub use crate::auth::paths::*;

fn main() {
    pretty_env_logger::init();
    match http::HttpClient::new() {
        Ok(_c) => {
            ()
        }
        Err(_) =>
            logger::warn("API_TOKEN is not configured")
    }
}
