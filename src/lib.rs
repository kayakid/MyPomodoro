
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

    // sorted asc price ranges and gears at extremities
    pub g_i: Vec<GearRange>,

    //higher then p_n
    pub p_n: f64,
    pub g_n: f64,

}

impl Gear {

    pub fn positive(price0: f64, price1: f64) -> Self {
        Self {
            p_0: price0,
            g_0: 1.0,
            g_i: vec![GearRange{
                p_start: price0,
                g_start: 1.0,
                p_end: price1,
                g_end: 0.0,
            }],
            p_n: price1,
            g_n: 0.0,
        }
    }

    pub fn negative(price0: f64, price1: f64) -> Self {
        Self {
            p_0: price0,
            g_0: 0.0,
            g_i: vec![GearRange{
                p_start: price0,