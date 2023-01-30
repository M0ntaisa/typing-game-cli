use std::io;
use rand::prelude::*;
use serde::Deserialize;
use tokio;
use reqwest::Client;
use serde_json::from_str;

#[derive(Deserialize)]
struct ApiResponse {
    words : Vec<String>
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let res = client
        .get("https://random-word-api.herokuapp.com/all")
        .send()
        .await
        .unwrap();

    let body = match res.text().await{
        Ok(body) => body,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    let data: ApiResponse = match from_str(&body) {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    let mut words = data.words.into_iter().take(5).collect::<Vec<String>>();
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
// fix the err