fn main() {
    let name: String = String::from("Devex"); 
    let ans = sum(1,2);
    let even:bool = is_even(10);

    println!("{}", ans);
    println!("{}", even);
    println!("First name - {}", name);
}

fn sum(a: u32,b: u32) -> u32 {
    return a+b;
}

fn is_even(a: u32) -> bool {
    return a%2 == 0; 
}