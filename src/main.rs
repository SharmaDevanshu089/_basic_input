//Takes Basic Input
use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a Number");
    io::stdin().read_line(&mut input).expect("There has been a error");
    println!("You have entered {}" , input);
}
