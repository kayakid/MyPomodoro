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
    pub cask: f