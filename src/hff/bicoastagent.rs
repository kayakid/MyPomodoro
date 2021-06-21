use serde::{Serialize,Deserialize};

use super::agents::{GearHedger,Agent, GAgent};
use super::account::OrderFill;
use super::quote::Tick;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GBiAgent {
    pub price: f64,
    pub span: f64,
    pub scale: f64,
    pub exposure: f64,
    pub target: f64
}

impl GBiAgent {
    pub 