// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::consts::MIST_API_BASE;
use crate::http::HttpClient;
use crate::logger::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// ref: https://api.mist.com/api/v1/docs/Org#org
///

#[derive(Serialize, Deserialize, Debug)]
pub struct Orgs(Vec<Org>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Org {
    pub name: String,
    #[serde(default)]
    pub session_expiry: Option<u16>,
    #[serde(default)]
    pub alarmtemplate_id: Option<String>,
    #[serde(default)]
    pub orggroup_ids: Option<Vec<String>>,
    #[serde(default = "default_allow_mist")]
    pub allow_mist: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgStats {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub orggroup_ids: Option<Vec<String>>,
    pub allow_mist: bool,
    pub num_inventory: u32,
    pub num_devices: u32,
    pub num_devices_connected: u16,
    pub num_devices_disconnected: u16,
    pub num_sites: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgSetting {
    pub id: String,
    #[serde(default = "default_disabled")]
    pub for_site: bool,
    pub site_id: String,
    pub org_id: String,
    pub created_time: u32,
    pub modified_time: u32,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OrgSettingParams {
    pub name: String,
    #[serde(default)]
    pub password_policy: Option<PasswordPolicy>,
    #[serde(default)]
    pub ui_idle_timeout: Option<u16>,
    #[serde(default)]
    pub mgmt: Option<Management>,
    #[serde(default)]
    pub disable_pcap: Option<bool>,
    #[serde(default)]
    pub pcap: Option<Pcap>,
    #[serde(default)]
    pub pcap_bucket_verified: Option<bool>,
    #[serde(default)]
    pub security: Option<Security>,
    #[serde(default)]
    pub installer: Option<Installer>,
    #[serde(default)]
    pub remote_syslog: Option<RemoteSyslog>,
    #[serde(default)]
    pub auto_site_assignment: Option<AutoSiteAssignment>,
    #[serde(default)]
    pub auto_device_naming: Option<AutoDeviceNaming>,
    #[serde(default)]
    pub cloudshark: Option<CloudShark>,
    #[serde(default)]
    pub auto_deviceprofile_assignment: Option<AutoDeviceProfile>,
    #[serde(default)]
    pub cacerts: Option<Vec<String>>,
    #[serde(default)]
    pub device_cert: Option<DeviceCert>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PasswordPolicy {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub freshness: u32,
    pub min_length: u32,
    #[serde(default = "default_disabled")]
    pub requires_special_char: bool,
    #[serde(default = "default_disabled")]
    pub requires_two_factor_auth: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Management {
    #[serde(default = "default_disabled")]
    pub use_wxtunnel: bool,
    #[serde(default = "default_disabled")]
    pub use_mxtunnel: bool,
    pub mxtunnel_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pcap {
    pub bucket: String,
    pub max_pkt_len: u16,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Security {
    #[serde(default = "default_disabled")]
    pub disable_local_ssh: bool,
    #[serde(default = "default_disabled")]
    pub limit_ssh_access: bool,
    pub fips_zeroize_password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Installer {
    pub grace_period: u32,
    pub extra_site_ids: Vec<String>,
    #[serde(default = "default_disabled")]
    pub allow_all_sites: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RemoteSyslog {
    #[serde(default = "default_disabled")]
    pub enabled: bool,
    #[serde(default = "default_disabled")]
    pub send_to_all_servers: bool,
    pub servers: Vec<RemoteSyslogServer>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RemoteSyslogServer {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub facility: String,
    pub severity: String,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AutoSiteAssignment {
    #[serde(default = "default_disabled")]
    pub enable: bool,
    pub rules: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AutoDeviceNaming {
    #[serde(default = "default_disabled")]
    pub enable: bool,
    pub rules: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloudShark {
    apitoken: String,
    #[serde(default)]
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AutoDeviceProfile {
    #[serde(default = "default_disabled")]
    pub enable: bool,
    pub rules: Vec<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cacerts(Vec<String>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceCert {
    pub cert: String,
    pub key: String,
}

pub fn get_setting<'a>(c: &HttpClient, org_id: &'a str) -> Result<OrgSetting, ()> {
    match c.get(org_setting_path(org_id), &()) {
        Ok(Some(orgs)) => {
            debug("org setting request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org setting request failed");
            Err(())
        }
    }
}

pub fn update_setting<'a>(
    c: &HttpClient,
    org_id: &'a str,
    params: &OrgSettingParams,
) -> Result<OrgSetting, ()> {
    match c.put(org_setting_path(org_id), params) {
        Ok(Some(orgs)) => {
            debug("org setting change request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org setting change request failed");
            Err(())
        }
    }
}

pub fn get_stats<'a>(c: &HttpClient, org_id: &'a str) -> Result<OrgStats, ()> {
    match c.get(org_stats_path(org_id), &()) {
        Ok(Some(orgs)) => {
            debug("org stats request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org stats request failed");
            Err(())
        }
    }
}

pub fn create(c: &HttpClient, org: &Org) -> Result<Org, ()> {
    match c.post(orgs_path(), org) {
        Ok(Some(orgs)) => {
            debug("org create request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org create request failed");
            Err(())
        }
    }
}

pub fn update<'a>(c: &HttpClient, org_id: &'a str, org: &Org) -> Result<Org, ()> {
    match c.put(org_path(org_id), org) {
        Ok(Some(orgs)) => {
            debug("org create request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org create request failed");
            Err(())
        }
    }
}

pub fn clone<'a>(c: &HttpClient, org_id: &'a str, name: &'a str) -> Result<Org, ()> {
    let mut req: HashMap<&str, &str> = HashMap::new();
    req.insert("name", name);
    match c.put(org_path(org_id), &req) {
        Ok(Some(orgs)) => {
            debug("org create request succeed");
            Ok(orgs)
        }
        _ => {
            warn("org create request failed");
            Err(())
        }
    }
}

/// private functions

fn orgs_path() -> String {
    format!("{}/orgs", MIST_API_BASE)
}

fn org_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}", MIST_API_BASE, org_id)
}

fn org_stats_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}/stats", MIST_API_BASE, org_id)
}

fn org_setting_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}/setting", MIST_API_BASE, org_id)
}

fn default_allow_mist() -> bool {
    true
}

fn default_enabled() -> bool {
    true
}

fn default_disabled() -> bool {
    true
}
