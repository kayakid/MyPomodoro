use serde::{Serialize,Deserialize};

use super::agents::{GearHedger,Agent, GAgent};
use super::account::OrderFill;
use super::quote::Tick;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GBiAgent {
    pub price: