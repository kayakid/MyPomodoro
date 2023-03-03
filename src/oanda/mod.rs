
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
    short: SideResponse,
}

#[derive(Deserialize, Debug)]
pub struct OpenPositionsResponse {
    positions: Vec<PositionsResponse>,
}

impl OpenPositionsResponse {
    pub fn to_position_vec(&self) -> Vec<Position> {
        let mut ret = Vec::new();
        for opr in self.positions.iter() {
            let units = if opr.long.units != "0"
            { opr.long.units.parse().unwrap()}
            else {  opr.short.units.parse().unwrap() };

            let price = if opr.long.units != "0"
            { opr.long.averagePrice.as_ref().map(|p| p.parse().unwrap())}
            else { opr.short.averagePrice.as_ref().map(|p| p.parse().unwrap()) };

            let position = Position {
                instrument: opr.instrument.clone(),
                units: units,
                price: price
            };
            ret.push(position);
        }
        ret
    }
}


#[derive(Deserialize, Debug)]
pub struct LiquidityResponse {
    price: String,
    liquidity: i64,
}
#[derive(Deserialize, Debug)]