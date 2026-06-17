use chrono::prelude::*;
use dotenv::dotenv;
use std::env;
// use chrono::{Utc, Local};

#[derive(Copy, Clone)]
struct Rect<T> {
    width: T,
    height: T
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height
    }
}

trait Shape {
    fn area(&self) -> u32;
}

fn get_area<T: Shape>(s: T) -> u32 {
    return s.area();
}
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

    let r = Rect{
        width: 10,
        height: 10
    };
    println!("{}", r.area());
    // println!("{}", var);
}