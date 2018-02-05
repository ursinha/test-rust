extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut attempts = 0;

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

            attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
    println!("Number of attempts: {}", attempts);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_two() {
        assert_ne!(2 + 2, 5);
    }
}
