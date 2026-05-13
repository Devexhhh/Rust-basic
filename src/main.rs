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
    // get_length(str); // moved the value, so OWNERSHIP is with get_length
}

fn get_length(str: String) -> usize {
    return str.len();
}
