use std::fs;

struct  Rect {
    height: f32,
    width: f32
}

impl Rect {
    fn area(&self) -> f32 {
        return self.height * self.width;
    }

    fn print_something() {
        println!("Static function")
    }
}

enum Direction {
    North, South, East, West
}

fn main() {
    let mut s1 = String::from("Devex");
    let s2 = &mut s1;
    
    s2.push_str(" Daddy");

    let s3 = &s1;
    let s4 = &s1;

    println!("{}, {}", s3, s4);
    

    let r = Rect {
        width: 10.0,
        height: 10.0,
    };

    println!("{}, {}", r.height, r.width);
    println!("{}", r.area());
    Rect::print_something();


    let direction = Direction::East;
    steer(direction);

    let contents = fs::read_to_string("a.txt");

    match contents {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error while reading file")
    }
    // get_length(str); // moved the value, so OWNERSHIP is with get_length
}

fn steer(dir: Direction) {
    match dir {
        Direction::North => print!("North Direction\n"),
        Direction::South => print!("SouthDirection\n"),
        _ => println!("Horizontal Direction")
    }
}

fn get_length(str: String) -> usize {
    return str.len();
}