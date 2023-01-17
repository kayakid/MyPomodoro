
use serde::{Serialize,Deserialize};

pub mod hff;
pub mod oanda;

// GearRange defines exposure gear linear between price limits
#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct GearRange {
    pub p_start: f64,
    pub g_start: f64,
    pub p_end: f64,
    pub g_end: f64,
}

impl GearRange {
    fn g(&self, x: f64) -> f64 {
        self.g_start + (x - self.p_start)*(self.g_end - self.g_start)/(self.p_end - self.p_start)
    }
}

// Gear defines gear below and above extreme prices and a vector of gears for specified intervals
#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct Gear {
    // lower than p_0
    pub p_0: f64,
    pub g_0: f64,