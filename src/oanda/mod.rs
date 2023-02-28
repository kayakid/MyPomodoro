
use serde::{Serialize,Deserialize};
use chrono::DateTime;
use super::hff::quote::*;
use super::hff::account::*;

pub mod client;

#[derive(Deserialize, Debug)]
pub struct SideResponse {
    units: String,
    averagePrice: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PositionsResponse {
    instrument: String,
    long: SideResponse,