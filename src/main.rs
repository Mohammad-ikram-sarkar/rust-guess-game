
use std::io;
use rand::Rng;

fn main () {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("this secret_number is : {secret_number}");
    println!("give me input");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to  read line");
    println!("your gueess : {guess}");
}