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
    name: String,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
    #[serde(default)]
    secpolicy_id: Option<String>,
    #[serde(default)]
    alarmtemplate_id: Option<String>,
    #[serde(default)]
    networktemplate_id: Option<String>,
    #[serde(default)]
    latlng: Option<HashMap<String, f64>>,
    #[serde(default)]
    sitegroup_ids: Option<Vec<String>>,
    #[serde(default)]
    address: Option<String>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteStats {
    name: String,
    id: String,
    #[serde(default)]
    address: Option<String>,
    #[serde(default)]
    alarmtemplate_id: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
    #[serde(default)]
    created_time: Option<u32>,
    #[serde(default)]
    lat: Option<f64>,
    #[serde(default)]
    latlng: Option<HashMap<String, f64>>,
    #[serde(default)]
    lng: Option<f64>,
    #[serde(default)]
    modified_time: Option<u32>,
    #[serde(default)]
    msp_id: Option<String>,
    #[serde(default)]
    networktemplate_id: Option<String>,
    #[serde(default)]
    num_ap: u16,
    #[serde(default)]
    num_ap_connected: u16,
    #[serde(default)]
    num_clients: u16,
    #[serde(default)]
    num_devices: u16,
    #[serde(default)]
    num_devices_connected: u16,
    #[serde(default)]
    num_gateway: u16,
    #[serde(default)]
    num_gateway_connected: u16,
    #[serde(default)]
    num_switch: u16,
    #[serde(default)]
    num_switch_connected: u16,
    #[serde(default)]
    org_id: Option<String>,
    #[serde(default)]
    rftemplate_id: Option<String>,
    #[serde(default)]
    secpolicy_id: Option<String>,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    tzoffset: Option<i32>,
}

pub fn get_stat<'a>(c: &HttpClient, site_id: &'a str) -> Result<SiteStats, ()> {
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
    println!("{}", site_path(site_id));
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
