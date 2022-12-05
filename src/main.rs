use std::{io};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, please input your guess");
    

    let input = get_user_input();
    let random_number = rand::thread_rng().gen_range(1..=100);
    compare_numbers(input, random_number)


    
}


fn get_user_input() -> i32 {
    let mut input: String  = String::new();
    return loop {
        println!("number must be 1 - 100");
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                println!("Input was accepted, not checked");
                
            },
            Err(err) => {
                println!("There was an error: {}", err.to_string());
                continue;
            }
        };
        println!("checking the input");
        
        match input.trim().parse::<i32>() {
            Ok(int) => {
                break int
            },
            Err(err) => {
                println!("There was an error in checking process {}", err.to_string());
                continue;
            }

        }
    }
    
    

}

fn compare_numbers( mut guess: i32, rand_num: i32) {
    match guess.cmp(&rand_num) {
        Ordering::Equal => {
            println!("You guess correctly");
            return;
        },
        Ordering::Greater => {
            println!("you guessed to high");
        },
        Ordering::Less => {
            println!("you guessed to low");

        }
    }

    println!("Would you like to try again?");
    println!("Input 'y' for yes or 'n' for no");

    let mut should_continue = String::new();
    loop {
        match io::stdin().read_line(&mut should_continue) {
            Ok(_n) => break,
            Err(err) => {
                println!("Error:  {}", err.to_string());
                continue;
            }
        }
    }
    if should_continue.trim() == "y" {

        guess = get_user_input();
        compare_numbers(guess, rand_num);
    }
    

}
