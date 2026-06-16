// use chrono::prelude::*;
use chrono::{Utc, Local};

fn main() {
    let utc = Utc::now();
    let local_time = Local::now();
    print!("{}", utc);
    print!("{}", local_time);
}