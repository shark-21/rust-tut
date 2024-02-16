use std::io;
 use rand::Rng;
use  std::cmp::Ordering ;

fn main() {
    println!("Hello, world!\n");
    
    //guess="shar256";
   
   
    let g_no = rand::thread_rng().gen_range(1..=100);
    // println!("{g_no} no.");
    // println!("{g_no} no.");
    let msg   = "Please type a number!";
    
    println!("Enter a number: ");
    let mut score: i32 = 0;
    loop{
        let mut guess  = String::new();
        io::stdin()
        .read_line(&mut guess).expect("Failed to read line");
    let guess : u32 =guess.trim().parse().expect(msg);
       
    match guess.cmp(&g_no) {
        Ordering::Greater => print!("high"),
        Ordering::Equal =>{
             print!("eq");
             print!("You win!!");
             println!("      --- score= {score} ");
             break;
    },
        Ordering::Less => println!("less"),

    }
    score=score+1;
    println!("      --- score= {score} ");
}

}
