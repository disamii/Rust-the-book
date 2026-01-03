use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

fn main() {
    println!("Guess the number!");
    let random = rand::rng().random_range(1..=100);
    loop {
        println!("Please input your guess.");
            let mut input = String::new();

        io::stdin().read_line(&mut input).expect("failed");
        println!("You guessed: {input}");
        let input: u32 = input.trim().parse().expect("enter correct number");

        match random.cmp(&input) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        };
    }
}
