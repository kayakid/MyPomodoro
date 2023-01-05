/* quotes, ticks and other pricing stuff */
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Bar {
    pub time: u64,
    obid: f64,
   