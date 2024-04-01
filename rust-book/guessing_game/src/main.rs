use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new(); // return a new String object, and assigned it to variable 'guess'

        io::stdin()
            .read_line(&mut guess) // & means reference
            .expect("Failed to read line"); 
            // read_line will pass the input to guess, 
            // and also return a Result type value, with enum(Ok, Err)
            // Result type has .expect method, if it gets err, the program stops
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        
        println!("Your guessed:{guess}");
        match guess.cmp(&secret_number){ 
            // match has arms, and each arm have patterns
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}