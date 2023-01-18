
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
                g_start: 0.0,
                p_end: price1,
                g_end: -1.0,
            }],
            p_n: price1,
            g_n: -1.0,
        }
    }

    pub fn constant(dir: i64) -> Self {
        Self {
            p_0: 1.0,
            g_0: if dir > 0 { 1.0} else { -1.0 },
            g_i: vec![],
            p_n: 1.0,
            g_n: if dir > 0 { 1.0} else { -1.0 },
        }
    }

    //
    pub fn symmetric(price0: f64, price1:f64) -> Self {
        Self {
            p_0: price0,
            g_0: 1.0,
            g_i: vec![GearRange{
                p_start: price0,
                g_start: 1.0,
                p_end: price1,
                g_end: -1.0,
            }],
            p_n: price1,
            g_n: -1.0,
        }
    }

    pub fn segment(price0: f64, g_0: f64, price1: f64, g_1: f64) -> Self {
        Self {
            p_0: price0,
            g_0: g_0,
            g_i: vec![GearRange{
                p_start: price0,
                g_start: g_0,
                p_end: price1,
                g_end: g_1,
            }],
            p_n: price1,
            g_n: g_1,
        }
    }

    pub fn jump(price0: f64, g_0: f64, g_n: f64) -> Self {
        Self {
            p_0: price0,
            g_0: g_0,
            g_i: vec![],
            p_n: price0,
            g_n: g_n,
        }
    }

    pub fn coastline(direction: i64, price0: f64, scale: f64, imax: f64) -> Self {
        if direction > 0 {
            Self {
                p_0: price0 - imax*scale,
                g_0: 1.0,
                g_i:vec![GearRange{
                    p_start: price0 - imax*scale,
                    g_start: 1.0,
                    p_end: price0 + scale,
                    g_end: 0.0,
                }],
                p_n: price0 + scale,
                g_n: 0.0,
            }
        } else {
            Self {
                p_0: price0 - scale,
                g_0: 0.0,
                g_i:vec![GearRange{
                    p_start: price0 - scale,
                    g_start: 0.0,
                    p_end: price0 + imax*scale,
                    g_end: -1.0,
                }],
                p_n: price0 + imax*scale,
                g_n: -1.0,
            }
        }
    }

    pub fn g(&self, x: f64) -> f64 {
        if x < self.p_0 {return self.g_0;}
        if x >= self.p_n {return self.g_n;}

        for g in self.g_i.iter() {
            if (x >= g.p_start && x < g.p_end) {
                return  g.g(x);
            }