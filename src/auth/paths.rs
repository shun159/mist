// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

pub const MIST_API_BASE: &'static str = "https://api.mist.com/api/v1";

/// ## Auth
/// ### Login/Logout/Lookup
pub const LOGIN: &'static str = "login";
pub const LOGIN_LOOKUP: &'static str = "login/lookup";
pub const LOGOUT: &'static str = "logout";

/// ### Privileges(Self)
pub const PRIV_WHOAMI: &'static str = "self";
/// ### Audit Logs
pub const AUDIT_LOG: &'static str = "self/logs";

pub fn login() -> String {
    format!("{}/{}", MIST_API_BASE, LOGIN)
}

pub fn logout() -> String {
    format!("{}/{}", MIST_API_BASE, LOGOUT)
}

pub fn get_privileges() -> String {
    format!("{}/{}", MIST_API_BASE, PRIV_WHOAMI)
}
