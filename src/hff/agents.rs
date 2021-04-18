
use serde::{Deserialize, Serialize};

use super::super::{Gear, GearRange};
use super::account::OrderFill;
use super::quote::Tick;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GAgent {
    OHLC {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        scale: f64,
        exposure: f64,
        target: Option<f64>,
    },
    // Coastline trader agent with parameters as defined in golang
    CL {
        direction: i64,
        price: f64,
        scale: f64,
        size: f64,
        i0: Option<f64>,
        imax: f64,