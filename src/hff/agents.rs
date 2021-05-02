
use serde::{Deserialize, Serialize};

use super::super::{Gear, GearRange};
use super::account::OrderFill;
use super::quote::Tick;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GAgent {
    OHLC {
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        scale: f64,
        exposure: f64,
        target: Option<f64>,
    },
    // Coastline trader agent with parameters as defined in golang
    CL {
        direction: i64,
        price: f64,
        scale: f64,
        size: f64,
        i0: Option<f64>,
        imax: f64,
        target: Option<f64>,
    },
    Symmetric {
        pmid: f64,
        span: f64,
        scale: f64,
        exposure: f64,
        target: f64,
    },
    Buy {
        price0: f64,
        price1: f64,
        scale: f64,
        exposure: f64,
    },
    Sell {
        price0: f64,
        price1: f64,
        scale: f64,
        exposure: f64,
    },
    JumpLong {
        price0: f64,
        scale: f64,
        exposure: f64,
    },
    // Coastline trader agent with parameters as defined in golang
    Coastline {
        direction: i64,
        price0: f64,
        scale: f64,
        size: f64,
        imax: f64,
    },
    Segment {
        price0: f64,
        exposure0: f64,
        pricen: f64,
        exposuren: f64,
        scale: f64,
        target: f64,
    },
}

impl GAgent {
    pub fn build(&self) -> Option<GearHedger> {
        match self {
            GAgent::OHLC {
                open: open,
                high: high,
                low: low,
                close: close,
                scale: scale,
                exposure: exposure,
                target: target,
            } => {
                // price to zero exposure
                let zerop = if open < close {close} else {open};
                // check where to set exposure at extremes
                let exposure0 = exposure.min(exposure * (zerop - low) / (high - zerop));
                let exposuren = - exposure.min(exposure * (high - zerop) / (zerop - low));
                let actualTarget = target.unwrap_or(f64::MAX);
                Some(GearHedger::segment(
                        *low, exposure0, *high, exposuren, *scale, actualTarget,
            ))
            },
            GAgent::CL {
                direction: direction,
                price: price,
                scale: scale,
                size: size,
                i0: i0,
                imax: imax,
                target: target,
            } => {
                let shift = i0.unwrap_or(1.0) * *scale;
                let zerop = if *direction > 0 { *price + shift} else { *price - shift };

                let high = zerop + imax * scale;
                let low = zerop - imax * scale;

                let actualTarget = target.unwrap_or(size * scale);
                let exposure = *size * *imax;

                Some(GearHedger::segment(
                        low, exposure, high, -exposure, *scale, actualTarget,
            ))
            },
            GAgent::Symmetric {
                pmid: pmid,
                span: span,
                scale: scale,
                exposure: exposure,
                target: target,
            } => Some(GearHedger::symmetric(
                    *pmid - *span,
                *pmid + *span,
                *scale,
                *scale,
                *exposure,
                *target,
            )),
            GAgent::Buy {
                price0: price0,
                price1: price1,
                scale: scale,
                exposure: exposure,
            } => Some(GearHedger::buyer(
                    *price0, *price1, *scale, *scale, *exposure,
            )),
            GAgent::Sell {
                price0: price0,
                price1: price1,
                scale: scale,
                exposure: exposure,
            } => Some(GearHedger::seller(
                    *price0, *price1, *scale, *scale, *exposure,
            )),
            GAgent::JumpLong {
                price0: price0,
                scale: scale,
                exposure: exposure,
            } => Some(GearHedger::jump(
                    *price0, 1.0, 0.0, *scale, *scale, *exposure,
            )),
            GAgent::Coastline {
                direction: direction,
                price0: price0,
                scale: scale,
                size: size,
                imax: imax,
            } => Some(GearHedger::coastline(
                    *direction, *price0, *scale, *size, *imax,
            )),
            GAgent::Segment {
                price0: price0,
                exposure0: exposure0,
                pricen: pricen,
                exposuren: exposuren,
                scale: scale,
                target: target,
            } => Some(GearHedger::segment(
                    *price0, *exposure0, *pricen, *exposuren, *scale, *target,
            )),