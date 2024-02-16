use std::io;
use rand::Rng;
extern crate rand;
// use  std::cmp::Ordering ;


fn main(){
    println!("\nYour name = ");

    let mut guess  = String::new();
    io::stdin()
    .read_line(&mut guess).expect("Failed to read line");
print!("{guess}",guess);
let guess : u32 =guess.trim().parse().expect("msg");
}