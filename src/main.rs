use chrono::prelude::*;
use dotenv::dotenv;
use std::env;
// use chrono::{Utc, Local};

fn main() {
    dotenv().ok();

    let utc = Utc::now();
    let local_time = Local::now();
    print!("{}", utc);
    print!("{}", local_time);

    let var = env::var("DATABASE_URL");
    match var {
        Ok(str) => println!("\n{}", str),
        Err(_e) => println!("\nError while reading variable")
    }
    // println!("{}", var);
}