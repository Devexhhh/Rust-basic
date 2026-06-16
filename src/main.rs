use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    print!("{}", utc);
}