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

#[derive(Deb