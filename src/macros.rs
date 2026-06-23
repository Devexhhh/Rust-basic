macro_rules! say_hello {
    () => {
        //declarative macros
        println!("Hello, World!");
    };
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    age: u32
}

fn main() {
    println!("Hello World");
    say_hello!();

    let u = User {
        username: String::from("Devex"),
        password: String::from("Devex"),
        age: 22
    };
    print!("{:?}", u); // debug

    let v = vec![1,2,3];
    print!("{:?}", v);
}