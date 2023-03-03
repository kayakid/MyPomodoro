
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
pub struct PriceResponse {
    time: String,
    bids: Vec<LiquidityResponse>,
    asks: Vec<LiquidityResponse>,
}
#[derive(Deserialize, Debug)]
pub struct PricingResponse {
    time: String,
    prices: Vec<PriceResponse>,
}
impl PricingResponse {
    pub fn get_tick(&self) -> Tick {
        Tick{
            time:  DateTime::parse_from_rfc3339(self.prices.first().map(|p| p.time.clone()).unwrap().as_str()).unwrap().timestamp(),
            bid: (self.prices.first().map(|p| p.bids.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
            ask: (self.prices.first().map(|p| p.asks.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct OrderRequestInner {
    units: String,
    instrument: String,
    timeInForce: String,
    #[serde(rename="type")]
    ordertype: String,
    positionFill: String,

}

#[derive(Serialize, Debug)]
pub struct OrderRequest {
    pub order: OrderRequestInner,
}

impl OrderRequest {
    pub fn new(units: i64, instrument: String) -> Self {
        Self {
            order: OrderRequestInner {
                units: units.to_string(),
                instrument: instrument,
                timeInForce: "FOK".to_owned(),
                ordertype: "MARKET".to_owned(),
                positionFill: "DEFAULT".to_owned(),
            }
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct OrderFillTransactionResponse {
    pub price: String,
    pub units: String,
    #[serde(rename="type")]
    pub filltype: String,
}

#[derive(Deserialize, Debug)]
pub struct PostOrderResponse {
    orderFillTransaction: OrderFillTransactionResponse
}

impl PostOrderResponse {
    pub fn get_order_fill(&self) -> Option<OrderFill> {
        if self.orderFillTransaction.filltype != "ORDER_FILL" {
            None
        } else {
            Some(OrderFill {
                price: self.orderFillTransaction.price.parse::<f64>().unwrap(),
                units: self.orderFillTransaction.units.parse::<i64>().unwrap(),
            })
        }
    }
}