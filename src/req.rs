use futures_util::StreamExt;
use crossterm::style::Stylize;
use crate::structs::{RequestParams, GPTResponse, OutputState};

pub async fn query_ai(client: &reqwest::Client, api_key: &String, prompt: String, mt: i32) {
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
            if cj.starts_with("[DONE]") {
                println!("\n\n{} {}\n", crate::helpers::get_timestamp().yellow(), "[DONE]".blue());
                break;
            }

            let json: serde_json::Value = match serde_json::from_str(&cj) {
                Ok(val) => val,
                Err(e) => {
                    println!(
                        "\n\n{} [ERROR] {}\nWhile parsing response: {}\n\n", 
                        crate::helpers::get_timestamp().yellow(), 
                        e.to_string().red(), 
                        cj
                    );
                    serde_json::Value::String("".to_string())
                },
            };
            if json.to_string().len() == 0 {
                continue
            }

            let message = &json["choices"][0]["text"];
            match message.as_str()  {
                Some(val) => {
                    if val == "\n" {
                        print!("\n")
                    } else {
                        if gr.state == OutputState::CodeBlock {
                            print!("{}", val.yellow());
                        } else {
                            print!("{}", val.blue());
                        }
                        gr.append(val.into()) 
                    }
                },
                None => print!(""),
            }
        }      
    }
}