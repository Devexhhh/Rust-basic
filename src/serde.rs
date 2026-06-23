use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String
}
fn main() {
    let s = String::from("{\"username\": \"Devex\", \"password\": \"123123\"}");
    // let u = User {
    //     username: String::from("Devex"),
    //     password: String::from("123123")
    // };
    let u: Result<User, serde_json::Error> = serde_json::from_str(&s);
    match u {
        Ok(user) => print!("{:?}\n", user),
        Err(_) => print!("There was an error")
    }
}