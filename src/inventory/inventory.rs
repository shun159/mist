// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::consts::MIST_API_BASE;
use crate::http::HttpClient;
use crate::logger::{debug, warn};
use serde::{Deserialize, Serialize};

///
/// ref: https://api.mist.com/api/v1/docs/Org#inventory
///

#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub created_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimDeviceReply {
    #[serde(default)]
    pub added: Vec<String>,
    #[serde(default)]
    pub duplicated: Vec<String>,
    #[serde(default)]
    pub error: Vec<String>,
    #[serde(default)]
    pub inventory_added: Inventories,
    #[serde(default)]
    pub inventory_duplicated: Inventories,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DelInventory {
    #[serde(default = "delete_op")]
    pub op: String,
    #[serde(default)]
    pub serials: Vec<String>,
    #[serde(default)]
    pub macs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AssignInventory {
    #[serde(default = "assign_op")]
    pub op: String,
    pub site_id: String,
    pub macs: Vec<String>,
    #[serde(default)]
    pub no_reassign: Option<bool>,
    #[serde(default)]
    pub disable_auto_config: Option<bool>,
    #[serde(default)]
    pub managed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnassignInventory {
    #[serde(default = "unassign_op")]
    pub op: String,
    pub macs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryReply {
    pub op: String,
    pub success: Vec<String>,
    pub error: Vec<String>,
}

pub fn list<'a>(
    c: &HttpClient,
    org_id: &'a str,
    query: Option<&'a str>,
) -> Result<Inventories, ()> {
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

pub fn claim<'a>(
    c: &HttpClient,
    org_id: &'a str,
    claims: Vec<&'a str>,
) -> Result<ClaimDeviceReply, ()> {
    match c.post(inventories_path(org_id, None), &claims) {
        Ok(Some(reply)) => {
            debug("Claim device request succeed");
            Ok(reply)
        }
        _ => {
            warn("Claim device request failed");
            Err(())
        }
    }
}

pub fn delete<'a>(
    c: &HttpClient,
    org_id: &'a str,
    serials: Vec<String>,
    macs: Vec<String>,
) -> Result<InventoryReply, ()> {
    let req = DelInventory {
        serials,
        macs,
        ..Default::default()
    };
    match c.put(inventories_path(org_id, None), &req) {
        Ok(Some(reply)) => {
            debug("delete inventory request succeed");
            Ok(reply)
        }
        _ => {
            warn("delete inventory request failed");
            Err(())
        }
    }
}

pub fn assign<'a>(
    c: &HttpClient,
    org_id: &'a str,
    site_id: &'a str,
    macs: Vec<String>,
    no_reassign: Option<bool>,
    disable_auto_config: Option<bool>,
    managed: Option<bool>,
) -> Result<InventoryReply, ()> {
    let req = AssignInventory {
        site_id: site_id.to_string(),
        macs: macs,
        no_reassign: no_reassign,
        disable_auto_config: disable_auto_config,
        managed: managed,
        ..Default::default()
    };
    match c.put(inventories_path(org_id, None), &req) {
        Ok(Some(reply)) => {
            debug("assign inventory request succeed");
            Ok(reply)
        }
        _ => {
            warn("assign inventory request failed");
            Err(())
        }
    }
}

pub fn unassign<'a>(
    c: &HttpClient,
    org_id: &'a str,
    macs: Vec<String>
) -> Result<InventoryReply, ()> {
    let req = UnassignInventory { macs: macs, ..Default::default() };
    match c.put(inventories_path(org_id, None), &req) {
        Ok(Some(reply)) => {
            debug("unassign inventory request succeed");
            Ok(reply)
        }
        _ => {
            warn("unassign inventory request failed");
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

fn delete_op() -> String {
    "delete".to_string()
}

fn assign_op() -> String {
    "assign".to_string()
}

fn unassign_op() -> String {
    "unassign".to_string()
}
