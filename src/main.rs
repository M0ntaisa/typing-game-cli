use std::io;
use rand::prelude::*;

fn main() {
    let mut words = vec!["hello", "world", "typing", "game"];
    let mut score = 0;

    let mut rng = thread_rng();
    
    loop {
        words.shuffle(&mut rng);
        for word in &words {
            println!("Type the word: {}", word);
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
    
            if input.trim() == word.to_string() {
                println!("Correct!");
                score += 1;
            } else {
                println!("Incorrect. The correct word was: {}", word);
            }
        }
    
        println!("Your final score is: {}", score);
        println!("Play Again?: (yes/no) ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "no" {
            break;
        }
        score = 0;
    }
}

// TODO 
// make it loop
// use api for the words