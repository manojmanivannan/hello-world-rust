#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    
    let secret = rand::thread_rng().gen_range(1..=10);
    // println!("The secret number is {secret}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    let guess1: u32 = guess.trim().parse().expect("Please type a number");
    
    println!("You guessed: {guess1}");
    match guess1.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("MATCH!"),
    }
    
}

#[test]
fn test_main() {
    assert_eq!(1,1);
    assert_eq!(4,4);
}