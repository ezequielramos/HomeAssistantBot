#[macro_use]
extern crate log;
extern crate serenity;

extern crate dotenv;
extern crate env_logger;

mod rosie;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    env_logger::init();

    info!(".env loaded");

    let token = &env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env");

    info!("Starting bot");

    rosie::start(token);
}
