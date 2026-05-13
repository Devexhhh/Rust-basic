fn main() {
    let str = String::from("Devex");
    let len = get_length(str);

    println!("{}", len);
    
    // get_length(str); // moved the value, so OWNERSHIP is with get_length
}

fn get_length(str: String) -> usize {
    return str.len();
}
