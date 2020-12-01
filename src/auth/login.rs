// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::auth::paths;
use crate::http::HttpClient;
use crate::logger::{debug, info, warn};
use serde::Deserialize;
use serde_json::Value::Object;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Whoami {
    email: String,
    first_name: String,
    last_name: String,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    via_sso: Option<String>,
    privileges: Vec<Privilege>,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Privilege {
    scope: String,
    org_id: String,
    #[serde(default)]
    org_name: Option<String>,
    #[serde(default)]
    msp_id: Option<String>,
    #[serde(default)]
    msp_name: Option<String>,
    #[serde(default)]
    orggroup_ids: Option<Vec<String>>,
    name: String,
    role: String,
    #[serde(default)]
    site_id: Option<String>,
    #[serde(default)]
    sitegroup_ids: Option<Vec<String>>,
}

pub fn login<'a>(c: &HttpClient, u: &'a str, p: &'a str, otp: Option<&'a str>) -> Result<(), ()> {
    let req = login_request(u, p, otp);
    match c.post(paths::login(), &req) {
        Ok(Some(Object(map))) if map.is_empty()  => {
            info("Login succeed");
            Ok(())
        }
        _ => {
            warn("login failed: the credentials incorrect");
            Err(())
        }
    }
}

pub fn logout(c: &HttpClient) -> Result<(), ()> {
    match c.post(paths::logout(), &()) {
        Ok(_) => {
            info("Logout succeed");
            Ok(())
        },
        Err(_) => {
            warn("logout failed");
            Err(())
        },
    }
}

pub fn get_priviledges(c: &HttpClient) -> Result<Whoami, ()> {
    match c.get(paths::get_privileges(), &()) {
        Ok(Some(whoami)) => {
            debug("get_privileges request succeed");
            Ok(whoami)
        }
        _ => {
            warn("get_privileges request failed");
            Err(())
        }
    }
}

/// private functions

fn login_request<'a>(n: &'a str, p: &'a str, otp: Option<&'a str>) -> HashMap<String, String> {
    let mut req: HashMap<String, String> = HashMap::new();
    req.insert("email".to_string(), n.to_string());
    req.insert("password".to_string(), p.to_string());
    if let Some(mfa_code) = otp {
        req.insert("two_factor".to_string(), mfa_code.to_string());
    }
    req
}
