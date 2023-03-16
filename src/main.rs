use crossterm::style::Stylize;
use rust_gpt_cli::{env, req, log, helpers, menu, structs::{RequestParams, GPTModel, ChatMessages}};
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
    let sm = env::get_system_message();
    let client = reqwest::Client::new();
    let mut rl = Editor::<()>::new().unwrap();

    if rl.load_history("history.txt").is_err() {
        log::log_info("No previous history.")
    }

    menu::main_menu();

    let mut rp: RequestParams = RequestParams::new(&sm, mt);
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
                // Keyword commands
                if l == "exit" {
                    println!("[rust_gpt_cli] exited");
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
                            rp.messages = ChatMessages::new(&sm);
                            println!("{} changed to {}", "Model".magenta(), &rp.model.clone().magenta());
                            continue;
                        },
                    }
                }

                // Handle calls to chat endpoint when a gpt model is selected
                if helpers::is_chat_model(&rp.model) {
                    rp.messages.add_user_msg(&line);
                    req::query_chat_completions(&client, &mut rp, &api_key).await;
                    continue;
                }
                
                // Send request to completions
                rp.prompt = line.clone();
                req::query_completions(&client, &rp, &api_key).await;
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
    rl.save_history("history.txt").unwrap();
}
