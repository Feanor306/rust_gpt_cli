use crossterm::style::Stylize;
use rust_gpt_cli::{env, req, log,helpers};
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[tokio::main]
async fn main() {
    let _l = log::init_logger().unwrap();
    let api_key = env::get_api_key();
    if api_key.len() == 0 {
        return;
    }
    
    let mt = env::get_max_tokens();
    let client = reqwest::Client::new();
    let mut rl = Editor::<()>::new().unwrap();
    // #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    print_menu();
    
    loop {
        let p = format!("{} {}", helpers::get_timestamp().yellow() ,"[PROMPT]: ".green());
        let prompt = rl.readline(&p);
        match prompt {
            Ok(line) => {
                let l = line.to_string();
                // Ignore empty prompt
                if l.trim().len() == 0 {
                    continue;
                }
                if l == "exit" {
                    println!("Program exited");
                    break;
                }
                if l == "model" {
                    println!("Not implemented yet");
                    continue;
                }
                req::query_ai(&client, &api_key, l, mt).await;
                rl.add_history_entry(&line);
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
    // #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt").unwrap();
}

fn print_menu() {
    // clear terminal screen on startup and move cursor to top
    print!("\x1B[2J\x1B[1;1H");

    // Main menu
    println!("\n###############################");
    println!("\n#####   {}   ######", "[rust_gpt_cli]".blue());
    println!("\n###############################");
    println!("\n###   {}   ###", "Available Commands:".blue());
    println!("\n {} : {}  ", "exit".red(), "terminate program");
    println!("\n {} : {} (default: {})  ", "model".green(), "choose GPT model", "text-davinci-002".yellow());
    println!("\n");
}