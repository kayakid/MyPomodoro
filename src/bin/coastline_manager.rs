
extern crate gear_trading;

use clap::{arg, command, Parser};
use std::fs;
use std::{thread, time};

use chrono::DateTime;
use chrono::Utc;
use error_chain::error_chain;
use serde::Deserialize;
use serde_json::json;
use std::env;
//use reqwest::Client;
use gear_trading::hff::account::*;
use gear_trading::hff::agents::*;
use gear_trading::hff::quote::Tick;
use gear_trading::oanda::client::Client;
use gear_trading::oanda::*;

use gear_trading::hff::overshoot::SpectrumClient;
use std::error::Error;
use tokio::main;

/*
error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}
*/
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the hedger file
    #[arg(short = 'f', long)]
    hedger_file: Option<String>,

    #[arg(short = 's', long)]
    spectrum_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let spectrum_url = args.spectrum_url;
    let spectrum_client = SpectrumClient::new(spectrum_url);

    //let cp = args.hedgerfile.as_deref();
    let hedger_opt = args
        .hedger_file
        .as_deref()
        .map(|f| {
            let hstr = fs::read_to_string(f).ok();
            hstr.map(|s| serde_json::from_str::<AgentInventory<GearHedger>>(s.as_str()).ok())
                .flatten()
        })
        .flatten();

    if hedger_opt.is_none() {}

    let delay = time::Duration::from_secs(15);
    let mut iter = 0;

    let oanda_url = env::var("OANDA_URL")?;
    let oanda_account = env::var("OANDA_ACCOUNT")?;