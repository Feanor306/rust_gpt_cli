use crossterm::style::Stylize;
use rust_gpt_cli::{env, req, helpers};
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[tokio::main]
async fn main() {
    let api_key = env::get_api_key();
    if api_key.len() == 0 {
        return;
    }
    
    let mt = env::get_max_tokens();
    let client = reqwest::Client::new();
    let mut rl = Editor::<()>::new().unwrap();
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    println!("\n#####   {}   ######", "[rust_gpt_cli]".blue());
    println!("\n## Enter {} or {} ##", "prompt".green(), "terminate".red());
    
    loop {
        let p = format!("{} {}", helpers::get_timestamp().yellow() ,"[PROMPT]: ".green());
        let prompt = rl.readline(&p);
        match prompt {
            Ok(line) => {
                if line == "exit" {
                    println!("Program exited");
                    break;
                }
                req::query_ai(&client, &api_key, line.to_string(), mt).await;
                rl.add_history_entry(line.as_str());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
}
