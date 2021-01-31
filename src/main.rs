extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    println!("Loading env vars from .env file...");
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
