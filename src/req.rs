use futures_util::StreamExt;
use crossterm::style::Stylize;
use crate::structs::RequestParams;

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
    
    let mut stream = res.unwrap().bytes_stream();
    let mut code_fmt: bool = false;
    println!("\n{} {}", crate::helpers::get_timestamp().yellow(), "[GPT]:".blue());
    while let Some(item) = stream.next().await {
        let bs = item.unwrap();
        // Strip first 6 characters "data: {valid_json_obj}"
        let json_string = String::from_utf8(bs[6..].to_vec()).unwrap();

        if json_string.starts_with("[DONE]") {
            println!("\n\n{} {}\n", crate::helpers::get_timestamp().yellow(), "[DONE]".blue());
            break;
        }

        // let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let json: serde_json::Value = match serde_json::from_str(&json_string) {
            Ok(val) => val,
            Err(e) => {
                println!(
                    "\n\n{} [ERROR] {}\nWhile parsing response: {}\n\n", 
                    crate::helpers::get_timestamp().yellow(), 
                    e.to_string().red(), 
                    json_string
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
                    // TODO find a better way to process this
                    if val.contains("'''") 
                    || val.contains("<code>") 
                    || val.contains("</code>"){
                        code_fmt = !code_fmt;
                    }
                    if code_fmt {
                        print!("{}", val.yellow());
                    } else {
                        print!("{}", val.blue());
                    }
                }
            },
            None => print!(""),
        }
    }
}