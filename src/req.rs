use std::io::prelude::*;                                                           
use std::io;   
use futures_util::StreamExt;
use crossterm::style::Stylize;
use crate::structs::{RequestParams, GPTResponse, OutputState};

pub async fn query_ai(client: &reqwest::Client, api_key: &String, prompt: String, mt: i32) {
    crate::log::log_info(format!("[PROMPT]: {}", prompt).as_str());
    let rp = RequestParams {
        prompt: prompt,
        model: "text-davinci-002".into(),
        stream: true,
        max_tokens: mt
    };
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&rp)
        .send()
        .await;
    
    let mut gr = GPTResponse::new();
    let mut stream = res.unwrap().bytes_stream();
    println!("\n{} {}", crate::helpers::get_timestamp().yellow(), "[GPT]:".blue());
    while let Some(item) = stream.next().await {
        let bs = item.unwrap();
        let json_string = String::from_utf8(bs.to_vec()).unwrap();
        let clean_json = crate::helpers::sanitize_json(&json_string);

        for cj in clean_json {
            let msg: String = crate::helpers::extract_msg_from_json(&cj);
            if msg.len() == 0 {
                continue;
            }

            match msg.as_str() {
                "\n" => println!(""),
                _ => {
                    if gr.state == OutputState::CodeBlock {
                        print!("{}", msg.as_str().yellow());
                    } else {
                        print!("{}", msg.as_str().blue());
                    }
                    gr.append(msg.clone()); 
                    // Flush stdout after each print! 
                    io::stdout().flush().ok().expect("Could not flush stdout");
                },
            }
        }      
    }
    // println! after print! ensures proper screen flush to avoid anomalies
    println!("\n\n{} {}\n", crate::helpers::get_timestamp().yellow(), "[DONE]".blue());
    crate::log::log_info(format!("[GPT]: {}", gr.payload).as_str());
}