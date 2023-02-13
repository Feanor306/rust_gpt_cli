use crossterm::style::Stylize;
use rust_gpt_cli::{env, req, log, helpers, menu, structs::{RequestParams, GPTModel}};
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

    menu::main_menu();

    let mut rp: RequestParams = RequestParams::new(mt);
    let mut vm: Vec<GPTModel> = vec!();
    
    loop {
        let p = format!("{}", helpers::entity_line(false, &rp.model, &"PROMPT".into()));
        let prompt = rl.readline(&p);
        match prompt {
            Ok(line) => {
                let l: String = line.to_string().trim().into();
                // Ignore empty prompt
                if l.len() == 0 {
                    continue;
                }
                if l == "exit" {
                    println!("Program exited");
                    break;
                }
                if l == "help" {
                    menu::main_menu();
                    continue;
                }
                if l == "model" {
                    vm = req::query_models(&client, &api_key).await;
                    menu::list_models(&vm);
                    continue;
                }
                if l.starts_with("model") {
                    let new_model = menu::change_model(&l, &vm);
                    match new_model.len() {
                        0 => continue,
                        _ => {
                            rp.model = new_model;
                            println!("{} changed to {}", "Model".magenta(), &rp.model.clone().magenta());
                            continue;
                        },
                    }
                }
                
                rp.prompt = line.clone();
                req::query_completions(&client, rp.clone(), &api_key).await;
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
