// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::consts::MIST_API_BASE;
use crate::http::HttpClient;
use crate::logger::{debug, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sites(Vec<Site>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub secpolicy_id: Option<String>,
    #[serde(default)]
    pub alarmtemplate_id: Option<String>,
    #[serde(default)]
    pub networktemplate_id: Option<String>,
    #[serde(default)]
    pub latlng: Option<HashMap<String, f64>>,
    #[serde(default)]
    pub sitegroup_ids: Option<Vec<String>>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteStats {
    pub name: String,
    pub id: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub alarmtemplate_id: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub created_time: Option<u32>,
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub latlng: Option<HashMap<String, f64>>,
    #[serde(default)]
    pub lng: Option<f64>,
    #[serde(default)]
    pub modified_time: Option<u32>,
    #[serde(default)]
    pub msp_id: Option<String>,
    #[serde(default)]
    pub networktemplate_id: Option<String>,
    #[serde(default)]
    pub num_ap: u16,
    #[serde(default)]
    pub num_ap_connected: u16,
    #[serde(default)]
    pub num_clients: u16,
    #[serde(default)]
    pub num_devices: u16,
    #[serde(default)]
    pub num_devices_connected: u16,
    #[serde(default)]
    pub num_gateway: u16,
    #[serde(default)]
    pub num_gateway_connected: u16,
    #[serde(default)]
    pub num_switch: u16,
    #[serde(default)]
    pub num_switch_connected: u16,
    #[serde(default)]
    pub org_id: Option<String>,
    #[serde(default)]
    pub rftemplate_id: Option<String>,
    #[serde(default)]
    pub secpolicy_id: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub tzoffset: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteGroups(Vec<SiteGroup>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteGroup {
    pub site_ids: Vec<String>,
    pub id: String,
    pub name: String,
    pub org_id: String,
    pub created_time: u32,
    pub modified_time: u32,
}

pub fn list_group<'a>(c: &HttpClient, org_id: &'a str) -> Result<SiteGroups, ()> {
    match c.get(site_groups_path(org_id), &()) {
        Ok(Some(sitegroups)) => {
            debug("list site groups request succeed");
            Ok(sitegroups)
        }
        _ => {
            debug("list site groups request failed");
            Err(())
        }
    }
}

pub fn get_group<'a>(c: &HttpClient, org_id: &'a str, group_id: &'a str) -> Result<SiteGroup, ()> {
    match c.get(site_group_path(org_id, group_id), &()) {
        Ok(Some(sitegroup)) => {
            debug("get site groups request succeed");
            Ok(sitegroup)
        }
        _ => {
            debug("get site groups request failed");
            Err(())
        }
    }
}

pub fn create_group<'a>(
    c: &HttpClient,
    org_id: &'a str,
    group: &SiteGroup,
) -> Result<SiteGroup, ()> {
    match c.post(site_groups_path(org_id), &group) {
        Ok(Some(sitegroup)) => {
            debug("create site groups request succeed");
            Ok(sitegroup)
        }
        _ => {
            debug("create site groups request failed");
            Err(())
        }
    }
}

pub fn update_group<'a>(
    c: &HttpClient,
    org_id: &'a str,
    group_id: &'a str,
    group: &SiteGroup,
) -> Result<SiteGroup, ()> {
    match c.put(site_group_path(org_id, group_id), &group) {
        Ok(Some(sitegroup)) => {
            debug("update site groups request succeed");
            Ok(sitegroup)
        }
        _ => {
            debug("update site groups request failed");
            Err(())
        }
    }
}

pub fn delete_group<'a>(c: &HttpClient, org_id: &'a str, group_id: &'a str) -> Result<(), ()> {
    match c.delete(site_group_path(org_id, group_id), &()) {
        Ok(Some(_)) => {
            debug("get site groups request succeed");
            Ok(())
        }
        _ => {
            debug("get site groups request failed");
            Err(())
        }
    }
}

pub fn get_stats<'a>(c: &HttpClient, site_id: &'a str) -> Result<SiteStats, ()> {
    match c.get(site_stats_path(site_id), &()) {
        Ok(Some(sites)) => {
            debug("get site stats request succeed");
            Ok(sites)
        }
        _ => {
            warn("get site stats request failed");
            Err(())
        }
    }
}

pub fn list<'a>(c: &HttpClient, org_id: &'a str) -> Result<Sites, ()> {
    match c.get(sites_path(org_id), &()) {
        Ok(Some(sites)) => {
            debug("list sites request succeed");
            Ok(sites)
        }
        _ => {
            warn("list sites request failed");
            Err(())
        }
    }
}

pub fn get<'a>(c: &HttpClient, site_id: &'a str) -> Result<Site, ()> {
    match c.get(site_path(site_id), &()) {
        Ok(Some(site)) => {
            debug("get site request succeed");
            Ok(site)
        }
        _ => {
            warn("get site request failed");
            Err(())
        }
    }
}

pub fn create<'a>(c: &HttpClient, org_id: &'a str, site: &Site) -> Result<Site, ()> {
    match c.post(sites_path(org_id), site) {
        Ok(Some(site)) => {
            debug("site creation is succeed");
            Ok(site)
        }
        _ => {
            warn("site creation is failed");
            Err(())
        }
    }
}

pub fn update<'a>(c: &HttpClient, site_id: &'a str, site: &Site) -> Result<Site, ()> {
    match c.put(site_path(site_id), site) {
        Ok(Some(site)) => {
            debug("site modification is succeed");
            Ok(site)
        }
        _ => {
            warn("site modification is failed");
            Err(())
        }
    }
}

pub fn delete<'a>(c: &HttpClient, site_id: &'a str) -> Result<(), ()> {
    match c.delete(site_path(site_id), &()) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn site_path<'a>(site_id: &'a str) -> String {
    format!("{}/sites/{}/stats", MIST_API_BASE, site_id)
}

pub fn site_stats_path<'a>(site_id: &'a str) -> String {
    format!("{}/sites/{}/stats", MIST_API_BASE, site_id)
}

pub fn sites_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}/sites", MIST_API_BASE, org_id)
}

pub fn site_groups_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}/sitegroups", MIST_API_BASE, org_id)
}

pub fn site_group_path<'a>(org_id: &'a str, group_id: &'a str) -> String {
    format!("{}/orgs/{}/sitegroups/{}", MIST_API_BASE, org_id, group_id)
}
