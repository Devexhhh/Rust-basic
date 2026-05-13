fn main() {
    let name: String = String::from("Devex"); 
    let ans = sum(1,2);
    let even:bool = is_even(10);
    let v: Vec<i32> = vec![1,2,3];


    let mut names:String = String::from("Devex");
    names.push_str(" Daddy");

    println!("{}", names);

    println!("{}", ans);
    println!("{}", even);
    println!("First name - {}", name);
    println!("{:?}", v);
    looper();
}

fn sum(a: u32,b: u32) -> u32 {
    return a+b;
}

fn is_even(a: u32) -> bool {
    return a%2 == 0; 
}

fn looper() {
    for i in 1..3 {
        println!("{}", i);
    }
}