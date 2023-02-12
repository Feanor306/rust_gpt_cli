use crossterm::style::Stylize;
use rust_gpt_cli::{env, req, log, helpers, structs::{RequestParams, GPTModel}};
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

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    print_menu();

    let mut rp: RequestParams = RequestParams::new(mt);
    let mut vm: Vec<GPTModel> = vec!();
    
    loop {
        let p = format!("{}", helpers::entity_line(false, &rp.model, &"PROMPT".into()));
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
                if l == "help" {
                    print_menu();
                    continue;
                }
                if l == "model" {
                    vm = req::query_models(&client, &api_key).await;
                    println!(" {} | {} | {}", "id".green(), "name".magenta(), "owned_by".blue());
                    for m in vm.iter() {
                        println!("{} | {} | {}", 
                            format!("{}", m.id).green(), 
                            m.name.clone().magenta(), 
                            m.owned_by.clone().blue()
                        );
                    }
                    println!("Type \"{} <{}>\" to change current model", "model".green(), "id".green());
                    continue;
                }
                if l.starts_with("model") {
                    if vm.is_empty() {
                        println!("Please call {} command at least once before trying to change {}.", 
                            "model".green(), 
                            "model".magenta(),
                        );
                        continue;
                    }
                    let id: i32 = l[5..].trim().parse().unwrap();

                    if id == 0 {
                        println!("Please use an existing <{}> from the list of {}.", "id".green(), "models".magenta());
                        continue;
                    }
                    for m in vm.iter() {
                        if m.id == id {
                            rp.model = m.name.clone();
                            println!("{} changed to {}", "Model".magenta(), &rp.model.clone().magenta());
                        }
                    }
                    continue;
                }
                
                rp.prompt = line.clone();
                req::query_completions(&client, rp.clone(), &api_key).await;
                rl.add_history_entry(&line);
            },
            Err(ReadlineError::Interrupted) => {
                rp.prompt = "hello".into();
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
    println!("\n {} : {} (default: {})  ", "model".green(), "list OpenAI models", "text-davinci-003".magenta());
    println!("\n {} <{}> : {}  ", "model".green(), "id".magenta(), "change model");
    println!("\n {} : {}  ", "help".blue(), "list commands");
    println!("\n");
}