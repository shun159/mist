// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use crate::logger;
use reqwest::{blocking::Client, header, header::AUTHORIZATION};
use serde::Serialize;
use std::option_env;

#[derive(Debug)]
pub struct HttpClient {
    c: Client,
}

impl HttpClient {
    pub fn new() -> reqwest::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(AUTHORIZATION, api_token().unwrap());
        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();
        Ok(HttpClient { c: client })
    }

    pub fn get<T, U>(&self, url: String, body: &T) -> reqwest::Result<Option<U>>
    where
        U: for<'de> serde::Deserialize<'de>,
        T: Serialize + ?Sized,
    {
        let request = self.c.get(&*url).json(body);
        let response: U = request.send()?.json()?;
        Ok(Some(response))
    }

    pub fn post<T>(&self, url: String, body: &T) -> reqwest::Result<Option<serde_json::Value>>
    where
        T: Serialize + ?Sized,
    {
        let request = self.c.post(&*url).json(body);
        let response: serde_json::Value = request.send()?.json()?;
        Ok(Some(response))
    }

    pub fn put<T>(&self, url: String, body: &T) -> reqwest::Result<Option<serde_json::Value>>
    where
        T: Serialize + ?Sized,
    {
        let request = self.c.put(&*url).json(body);
        let response: serde_json::Value = request.send()?.json()?;
        Ok(Some(response))
    }

    pub fn delete<T>(&self, url: String, body: &T) -> reqwest::Result<Option<serde_json::Value>>
    where
        T: Serialize + ?Sized,
    {
        let request = self.c.delete(&*url).json(body);
        let response: serde_json::Value = request.send()?.json()?;
        Ok(Some(response))
    }
}

fn api_token() -> Result<header::HeaderValue, ()> {
    match option_env!("MIST_TOKEN") {
        None => {
            logger::warn("env MIST_TOKEN is not configured");
            Err(())
        }
        Some(token) => {
            let token = &*format!("Token {}", token);
            let header = header::HeaderValue::from_str(token).unwrap();
            Ok(header)
        }
    }
}
