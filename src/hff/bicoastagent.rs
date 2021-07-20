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
    pub fn build(&self) -> BiCoastAgent {
        BiCoastAgent::new(self.price, self.span, self.scale, self.exposure, self.target)
    }
}

/*
BiCoastAgent is a symmetric GearHedger with specifications such that:
- an epoch_target is set as the profit target before we recalibrate the mid price
