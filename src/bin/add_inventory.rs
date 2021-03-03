extern crate gear_trading;

use clap::{arg, command, Parser};
use std::fs;
use gear_trading::hff::agents::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the hedger file
    #[arg(short = 'f', long)]
    hedger_file: Option<String>,

   #[arg(short = 'a', long)]
   agent: String,

    #[arg(short = 'n', long)]
   name: String,
}

fn main() {
    let args = Args::parse();

    //let cp = args.hedgerfile.as_deref();
    let mut hedger = args
        .hedger_file
        .as_deref()
        .map(|f| {
            let hstr = fs::read_to_string(f).ok();
            hstr.map(|s| serde_json::from_str::<AgentInventory<GearHedger>>(s.as_str()).ok())
                .flatten()
        })
        .flatten().unwrap();

    let agent = serde_json::from_str::<GAgent>(args.agent.as_str()).ok().unwrap();
    hedger.agents.insert(args.name.clone(), agent.build().unwrap());

    println!("{}", serde_json::to_string(&hedger).unwrap());
}