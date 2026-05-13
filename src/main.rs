fn main() {
    let mut s1 = String::from("Devex");
    let s2 = &mut s1;
    
    s2.push_str(" Daddy");

    let s3 = &s1;
    let s4 = &s1;

    println!("{} {}", s3, s4);
    
    // get_length(str); // moved the value, so OWNERSHIP is with get_length
}

fn get_length(str: String) -> usize {
    return str.len();
}
