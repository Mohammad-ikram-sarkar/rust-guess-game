
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main () {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("this secret_number is : {secret_number}");
    loop {
        println!("give me input");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to  read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("your gueess : {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {println!("You win!");
         break;
    }
        ,
    }
        
    }
    
}