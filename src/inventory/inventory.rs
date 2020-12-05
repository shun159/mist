// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::consts::MIST_API_BASE;
use crate::http::HttpClient;
use crate::logger::{debug, warn};
use serde::{Deserialize, Serialize};

///
/// ref: https://api.mist.com/api/v1/docs/Org#inventory
///

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventories(Vec<Inventory>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    // device serial
    pub serial: String,
    // device id
    pub id: String,
    // device model
    pub model: String,
    // device type, ap
    #[serde(rename = "type")]
    pub ty: String,
    // MAC address
    pub mac: String,
    // device name if assigned
    #[serde(default)]
    pub name: Option<String>,
    // site id if assigned, null if not assigned
    #[serde(default)]
    pub site_id: Option<String>,
    // deviceprofile id if assigned, null if not assigned
    #[serde(default)]
    pub deviceprofile_id: Option<String>,
    // device SKU
    #[serde(default)]
    pub sku: Option<String>,
    // device revision
    #[serde(default)]
    pub hw_rev: Option<String>,
    // magic
    #[serde(default)]
    pub magic: Option<String>,
    // device status
    pub connected: Option<bool>,
    // inventory last modified time, in epoch
    pub modified_time: u32,
    // inventory created time, in epoch
    pub created_time: u32
}

pub fn list<'a>(c: &HttpClient, org_id: &'a str, query: Option<&'a str>) -> Result<Inventories, ()> {
    match c.get(inventories_path(org_id, query), &()) {
        Ok(Some(inventories)) => {
            debug("list inventories request succeed");
            Ok(inventories)
        }
        _ => {
            warn("list inventories request failed");
            Err(())
        }
    }
}

/// private functions

fn inventories_path<'a>(org_id: &'a str, query: Option<&'a str>) -> String {
    if let Some(qstr) = query {
        format!("{}/orgs/{}/inventory?{}", MIST_API_BASE, org_id, qstr)
    } else {
        format!("{}/orgs/{}/inventory", MIST_API_BASE, org_id)
    }
}
