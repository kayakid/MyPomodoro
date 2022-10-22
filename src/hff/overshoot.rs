use serde::{Serialize,Deserialize};

// classes from the spectrum service
#[derive(Deserialize, Debug)]
pub struct os {
    pub instrument: String,
    pub scale: f64,
    pub direction: i64,
    pub start: f64,
    pub peak: f64,
    pu