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
    println!("\n{}:", "[GPT]".blue());
    while let Some(item) = stream.next().await {
        let bs = item.unwrap();
        let json_string = String::from_utf8(bs[6..].to_vec()).unwrap();

        if json_string.starts_with("[DONE]") {
            println!("\n\n{}", "[DONE]".blue());
            break;
        }

        let json: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let message = &json["choices"][0]["text"];
        match message.as_str()  {
            Some("\n") => print!("\n"),
            Some(val) => print!("{}", val.blue()),
            None => print!(""),
        }
    }
}