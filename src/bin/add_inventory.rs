extern crate gear_trading;

use clap::{arg, command, Parser};
use std::fs;
use gear_trading::hff::agents::*;

#[derive(Pars