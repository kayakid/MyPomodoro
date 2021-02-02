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
         