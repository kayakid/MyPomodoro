
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
    let mut cOS = os.maxOS();

    //println!("{:?}", spectrum_client.get().await.unwrap().to_spectrum());

    loop {
        // control loop counts and timing
        if iter != 0 {
            thread::sleep(delay);
        }
        iter = iter + 1;
        if iter > 10000 {
            break;
        }

        // now what we need to create a coastline agent
        let sp = spectrum_client.get().await.map(|s| s.to_spectrum());
        // if we get a spectrum from the service
        if let Some(os) = sp.map(|e| e.overshoots.iter().filter(|p| p.0 == 0.1).last().unwrap().1) {
            // reversal create a coastline agent
            // all these things should be config at startup...
            // scale: 0.0010  trading thresholds
            // size: 10000    trading size at scale
            // depth: 15      how many times size to accumulate
            // altitude: 15   how many times size would we accumulate on the other side
            // shift: 1       how many size are we shifted on entry
            if os.direction != cDir {
                let scale = 0.0010;
                let size = 10000.0;
                let target = scale * size;
                let price = os.current;
                let price0 = price - 15.0 * scale - os.direction.signum() as f64 * scale;
                let pricen = price + 15.0 * scale - os.direction.signum() as f64 * scale;
                let exposure0 = 15.0 * size;
                let exposuren = -15.0 * size;
                let mut agent = GAgent::Segment{price0: price0, exposure0: exposure0, pricen: pricen, exposuren: exposuren, scale: scale, target: 10.0}.build().unwrap();
                let key = format!("coastline_{}", if os.direction > 0 {"short"} else {"long"});
                eprintln!("Creating the agent on reversal: {:?}", agent);
                // TODO check the target agent key status to see if we add, or re-activate a new one
                if hedger.agents.get(&key).is_none() {
                    hedger.agents.insert(key, agent);
                }
            }
            // update the direction and current overshoot
            cDir = os.direction;
            cOS = os.maxOS();
        }

        // get the market tick
        let tick = client
            .get_pricing(String::from("EUR_USD"))
            .await
            .unwrap()
            .get_tick();

        // time now
        let now = Utc::now().timestamp();

        // check account positions
        let positions = client.get_open_positions().await.unwrap().to_position_vec();
        //println!("{:?}", positions);

        // compare target exposure with actual
        let target_exposure = hedger.next_exposure(&tick);
        let account_exposure = positions.iter().filter(|p| p.instrument == "EUR_USD").last().map_or_else(|| 0, |p| p.units);
        //println!("Target Exposure: {}", target_exposure);
        //println!("Actual Exposure: {}", account_exposure);

        // no trade
        if target_exposure == account_exposure {
            continue;
        }

        // create order
        let order = OrderRequest::new(target_exposure - account_exposure, "EUR_USD".to_string());

        eprintln!("Trading : {} to reach {} at price", target_exposure - account_exposure, target_exposure);

        client.post_order_request(&order).await.map_or(
                eprintln!("Cannot get the Post Order to Oanda, will try again next cycle"),
            |order_fill| {
                    order_fill.get_order_fill().map_or(
                            eprintln!("Cannot get the OrderFill from response, will try again next cycle"),
                    |of| {
                                hedger.update_on_fill(&of);
                                let hedger_str = serde_json::to_string(&hedger).ok().unwrap();
                                println!("{}", hedger_str);
                            },
                );
                },
        );
        // cleanup the closed agents
        hedger.agents.retain(|key, ga| {
            if ! ga.is_active() {
                eprintln!("Removing agent {} inactivated on PL: {:.2} and exposure {}", key, ga.agentPL.cum_profit, ga.exposure());
            }
            ga.active
        });
    }

    Ok(())
}