use serde::{Serialize,Deserialize};

use super::agents::{GearHedger,Agent, GAgent};
use super::account::OrderFill;
use super::quote::Tick;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GBiAgent {
    pub price: f64,
    pub span: f64,
    pub scale: f64,
    pub exposure: f64,
    pub target: f64
}

impl GBiAgent {
    pub fn build(&self) -> BiCoastAgent {
        BiCoastAgent::new(self.price, self.span, self.scale, self.exposure, self.target)
    }
}

/*
BiCoastAgent is a symmetric GearHedger with specifications such that:
- an epoch_target is set as the profit target before we recalibrate the mid price
- when the current tick leads to a cumulative profit + unrealized PL larger than the target,
we add the epoch target to the previous target, and the mid price becomes the current price.

*/
#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct BiCoastAgent {
    epoch_target: f64,
    gear_hedger: GearHedger,

}

impl BiCoastAgent {

    // constructor
    pub fn new(price: f64, span: f64, scale: f64, exposure: f64, target: f64) -> Self {
        let mut ret = Self {
            epoch_target: scale * exposure / span,
            gear_hedger: GAgent::Symmetric{pmid: price, span: span, scale: scale, exposure: exposure, target: target}.build().unwrap(),
        };
        ret.epoch_target = target;
        ret.gear_hedger.target = target;
        ret
    }


    fn mid_price(&self) -> f64 {
        (self.gear_hedger.gear_f.p_0 + self.gear_hedger.gear_f.p_n)/2.0
    }

    fn shift_mid_to_price(&mut self, price: f64) {
        let span = (self.gear_hedger.gear_f.p_n - self.gear_hedger.gear_f.p_0)/2.0;
        self.gear_hedger.gear_f =  GAgent::Symmetric{
            pmid: price,
            span: span,
            scale: self.gear_hedger.scaleUp,
            exposure: self.gear_hedger.max_exposure,
            target: self.gear_hedger.target}.build().unwrap().gear_f;
    }

    pub fn pl(&self) -> f64 {
        self.gear_hedger.agentPL.cum_profit
    }
}

impl Agent for BiCoastAgent {

    // NOT IMPLEMENTED!
    fn close(&mut self, tick :&Tick) -> i64 {
        // otherwize,we check if we need to adjust exposure

        0
    }

    fn is_active(&self) -> bool {
        true
    }
    fn deactivate(&mut self) {

    }

    // computes the status of the Agent: should it be closed
    fn to_be_closed(&self) -> bool {
        false
    }

    // specific to the bicoast agent if we reach the target
    fn target_action(&mut self) -> i64 {
        let price = self.gear_hedger.tentative_price;
        self.gear_hedger.target = self.gear_hedger.agentPL.pl_at_price(price) + self.epoch_target;
        self.shift_mid_to_price(price);
        return 0;
    }

    fn target_exposure(&mut self, tick: &Tick) -> i64 {
        self.gear_hedger.target_exposure(tick)
    }

    // compute the agent exposure if trading this tick
    fn next_exposure(&mut self, tick: &Tick) -> i64 {
        let clos