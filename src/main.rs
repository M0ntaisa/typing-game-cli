use std::io;

fn main() {
    let words = vec!["hello", "world", "typing", "game"];
    let mut score = 0;

    for word in words {
        println!("Type the word: {}", word);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == word {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect. The correct word was: {}", word);
        }
    }

    println!("Your final score is: {}", score);
}
