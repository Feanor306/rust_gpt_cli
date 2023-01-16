use std::io::{self, Write};
use crossterm::style::Stylize;
use rust_gpt_cli::{env, req};

#[tokio::main]
async fn main() {
    let api_key = env::get_api_key();
    if api_key.len() == 0 {
        return;
    }
    
    let mt = env::get_max_tokens();
    let client = reqwest::Client::new();

    println!("\n#####   {}   ######", "[rust_gpt_cli]".blue());
    println!("\n## Enter {} or {} ##", "prompt".green(), "terminate".red());

    loop {
        print!("\n{}: ", "[PROMPT]".green());
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).unwrap();

        req::query_ai(&client, &api_key, prompt, mt).await;
    }
}
