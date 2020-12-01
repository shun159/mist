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

pub fn get<'a>(c: &HttpClient, org_id: &'a str, site_id: &'a str) -> Result<Site, ()> {
    match c.get(site_path(org_id, site_id), &()) {
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

pub fn update<'a>(
    c: &HttpClient,
    org_id: &'a str,
    site_id: &'a str,
    site: &Site,
) -> Result<Site, ()> {
    match c.put(site_path(org_id, site_id), site) {
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

pub fn delete<'a>(c: &HttpClient, org_id: &'a str, site_id: &'a str) -> Result<(), ()> {
    match c.delete(site_path(org_id, site_id), &()) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn site_path<'a>(org_id: &'a str, site_id: &'a str) -> String {
    let base_path = sites_path(org_id);
    format!("{}/{}", base_path, site_id)
}

pub fn sites_path<'a>(org_id: &'a str) -> String {
    format!("{}/orgs/{}/sites", MIST_API_BASE, org_id)
}
