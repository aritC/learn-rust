use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=>{
                println!("Not a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }  
    }
}
