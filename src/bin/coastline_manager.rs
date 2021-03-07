
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
    let oanda_api_key = env::var("OANDA_API_KEY")?;

    let client = Client::new(
        oanda_url.clone(),
        oanda_account.clone(),
        oanda_api_key.clone(),
    );
    let mut hedger =
        hedger_opt.unwrap_or_else(|| {
            let mut inventory: AgentInventory<GearHedger> = AgentInventory::new();
            //inventory.agents.insert(String::from("shortloser"), GearHedger::symmetric(1.0150, 1.0650, 0.0010, 0.0010, 422500.0));
            inventory
        });
    //GearHedger::symmetric(1.0150, 1.0650, 0.0010, 422500.0));

    let hedger_str = serde_json::to_string(&hedger).ok().unwrap();
    println!("{}", hedger_str);

    //let mut inventory: AgentInventory<GearHedger> = AgentInventory::new();
    //inventory.agents.insert(String::from("shortloser"), hedger.clone());
    // inventory.agents.insert(String::from("bias"), DriftingHedge::new(4.0000, 422500, 2.0000));

    //let inv_str = serde_json::to_string(&inventory).ok().unwrap();
    //println!("{}", inv_str);
    let sp = spectrum_client.get().await.unwrap().to_spectrum();
    let os = sp.overshoots.iter().filter(|p| p.0 == 0.1).last().unwrap().1;

    let mut cDir = os.direction;