use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Wrong Input");
        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        println!("You Guessed {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Guessed it right");
                break;
            }
        }
    }

    
}
