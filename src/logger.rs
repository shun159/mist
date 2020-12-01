// Copyright 2020 Eishun Kondoh
// SPDX-License-Identifier: Apache-2.0

use log::{debug, error, info, warn};

pub fn debug<'a>(msg: &'a str) {
    debug!("{}", msg)
}

pub fn info<'a>(msg: &'a str) {
    info!("{}", msg)
}

pub fn warn<'a>(msg: &'a str) {
    warn!("{}", msg)
}

pub fn error<'a>(msg: &'a str) {
    error!("{}", msg)
}
