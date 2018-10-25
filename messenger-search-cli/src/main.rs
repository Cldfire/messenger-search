extern crate messenger_search;
#[macro_use]
extern crate structopt;
extern crate chrono;

use std::path::PathBuf;
use structopt::StructOpt;
use std::env;
use messenger_search::{search, generate_index, IndexStoreLocation::*};
use messenger_search::error::Error;
use chrono::{Local, TimeZone, Datelike};

#[derive(Debug, StructOpt)]
#[structopt(name = "msngr-search")]
struct Opt {
    /// Specifies the path to the message.json you want to search
    #[structopt(short = "i", parse(from_os_str))]
    input_json: Option<PathBuf>,
    #[structopt(parse(from_str))]
    query: String
}

fn main() -> Result<(), Error> {
    let mut opt = Opt::from_args();

    // TODO: We have to do this because we can't set a default to a function call with structopt
    if opt.input_json.is_none() {
        opt.input_json = Some(env::current_dir()?);
        opt.input_json.as_mut().unwrap().push("message.json");
    }

    // TODO: Don't store in memory
    let (idx, _) = generate_index(Ram, opt.input_json.unwrap().as_path())?;
    let search_results = search(&idx, &opt.query)?;

    for message in search_results {
        // TODO: Let user choose timezone to display
        let dt = Local.timestamp_millis(message.timestamp_ms[0]);
        println!(
            "<{} ({:?})> {}: {}",
            dt.format("%Y-%m-%d%l:%M %p"),
            dt.weekday(),
            message.sender_name[0],
            message.content[0]
        );
    }
    
    Ok(())
}
