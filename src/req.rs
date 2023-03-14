use std::io::prelude::*;                                                           
use std::io;
use futures_util::StreamExt;
use crossterm::style::Stylize;
use crate::{helpers, log, structs::{RequestParams, GPTResponse, GPTModel}};

pub async fn query_completions(client: &reqwest::Client, rp: RequestParams, api_key: &String) {
    crate::log::log_info(&format!("{}{}", helpers::entity_line(true, &rp.model, &"PROMPT".into()), &rp.prompt));

    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&rp)
        .send()
        .await;
    
    let mut gr = GPTResponse::new(&rp.prompt);
    let mut stream = res.unwrap().bytes_stream();
    println!("{}", helpers::entity_line(false, &rp.model, &"GPT".into()));
    while let Some(item) = stream.next().await {
        let bs = item.unwrap();
        let json_string = String::from_utf8(bs.to_vec()).unwrap();
        let clean_json = helpers::sanitize_json(&json_string, &rp.model);

        for cj in clean_json {
            let msg: String = helpers::extract_msg_from_json(&cj);
            if msg.len() == 0 {
                continue;
            }

            gr.append_full(msg.clone());

            // Check each char of message individually, 
            // because there could be multiple newlines 
            for c in msg.chars() { 
                match c {
                    '\n' => {
                        println!("");
                        gr.reset_line();
                    },
                    _ => {
                        print!("{}", format!("{}", c).blue());
                
                        // Flush stdout after each print!   
                        io::stdout().flush().ok().expect("Could not flush stdout");
                        gr.append_line(format!("{}", c));
                    },
                }
            }     
        }      
    }

    // println! after print! ensures proper screen flush to avoid anomalies
    println!("");
    // reset_line requires cursor to be at next line, must be done after a println!() or "\n"
    gr.reset_line();
    println!("{}", helpers::entity_line(false, &rp.model, &"DONE".into()));
    log::log_info(&format!("{}{}", helpers::entity_line(true, &rp.model, &"GPT".into()), gr.full_response));
    log::log_info(&format!("{}", helpers::entity_line(true, &rp.model, &"DONE".into())));
}

pub async fn query_models(client: &reqwest::Client, api_key: &String) -> Vec<GPTModel> {
    let res = client
        .get("https://api.openai.com/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    let msg = res.unwrap().text().await.unwrap();
    return crate::helpers::get_models_from_json(&msg);
}
