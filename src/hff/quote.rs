/* quotes, ticks and other pricing stuff */
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Bar {
    pub time: u64,
    obid: f64,
    hbid: f64,
    lbid: f64,
    pub cbid: f64,
    oask: f64,
    hask: f64,
    lask: f64,
    pub cask: f64,
}
impl Bar {
    pub fn time(&self) -> u64 {
        self.time / 1000
    }
    pub fn price(&self) -> f64 {
        (self.cbid+self.cask)/2.0
    }

    pub fn buy_price(&self) -> f64 {
        self.cask
    }
    pub fn sell_price(&self) -> f64 {
        self.cbid
    }

    pub fn spread(&self) -> f64 {
        self.cbid - self.cask
    }
}

#[derive(Debug)]
pub struct Tick {
    pub time: i64,
    pub bid: f64,
    pub ask: f64,
}

impl Tick {
    /* constructor from a Bar, we take close bid-ask */
    pub fn new(bar: &Bar) -> Self {
        Self {
            time: bar.time as i64,
            bid: bar.cbid,
            ask: bar.cask,
        }
    }

    pub fn time(&self) -> i64 {
        self.time / 1000
    }
    pub fn price(&self) -> f64 {
        (self.bid+self.ask)/2.0
    }

    pub fn buy_price(&self) -> f64 {
        self.ask
    }
    pub fn sell_price(&self) -> f64 {
        self.bid
    }

    pub fn spread(&self) -> f64 {
        self.bid - self.ask
    }
}