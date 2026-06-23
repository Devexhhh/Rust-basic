use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]

struct User {
    username: String,
    password: String
}

fn main() {
    let u = User {
        username: String::from("Devex"),
        password: String::from("123241")
    };

    let serialized_string = serde_json::to_string(&u);
    match serialized_string {
        Ok(str) => print!("{}\n", str),
        Err(_) => print!("Error while converting to string")
    }
}