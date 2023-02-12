use chrono::Local;
use crate::structs::GPTModel;
use crossterm::style::Stylize;

pub fn timestamp() -> String {
    return Local::now().format("%H:%M:%S").to_string();
}

pub fn timestamp_full() -> String {
    return Local::now().format("%d/%m/%y %H:%M:%S").to_string();
}

pub fn entity_line(full_time: bool, model: &String, entity: &String) -> String {
    let ts = match full_time {
        true => timestamp_full(),
        _ => timestamp(),
    };
    let et = match entity.as_str() {
        "PROMPT" => format!("{}{}{}:",
            "[".green(),
            entity.clone().green(),
            "]".green(),
        ),
        "DONE" => format!("{}{}{}", 
            "[".blue(),
            entity.clone().blue(),
            "]".blue(),
        ),
        _ => format!("{}{}{}:", 
            "[".blue(),
            entity.clone().blue(),
            "]".blue(),
        ),
    };

    let res = format!("\n{}{}{} {}{}{}",
            "[".yellow(),
            ts.yellow(),
            "]".yellow(),
            "[".magenta(),
            model.clone().magenta(),
            "]".magenta(),
    );

    match entity.len() {
        0 => return format!("{}: ", res),
        _ => return format!("{} {} ", 
            res,
            et,
        ),
    }
}

pub fn sanitize_json(raw_stream_data: &String, model: &String) -> Vec<String> {
    let mut sanitized = Vec::new();
    let mut rsd = raw_stream_data.clone();
    while rsd.contains("data: ") {
        rsd = rsd.replacen("data: ", "", 1);
        match parse_first_json(&rsd, model) {
            Some(obj) => {
                sanitized.push(obj.to_string());
                rsd = rsd[obj.len()..].to_string();
            },
            None => break,
        }
    }
    return sanitized
}

fn parse_first_json(input: &str, model: &String) -> Option<String> {
    let expected_end_of_json: String = "}], \"model\": \"".to_owned() + model + "\"}".into();
    let end = input.find(&expected_end_of_json)? + expected_end_of_json.len() + 1;

    match input[..end].to_string().len() {
        0 => None,
        _ => Some(input[..end].into()),
    }
}

pub fn extract_msg_from_json(cj: &String) -> String {
    if cj == "[DONE]" {
        return "[DONE]".into();
    }
    let json: serde_json::Value = match serde_json::from_str(&cj) {
        Ok(val) => val,
        Err(e) => {
            let msg = format!(
                "\n\n{} [ERROR] {}\nWhile parsing response: {}\n\n", 
                crate::helpers::timestamp_full(), 
                e.to_string(), 
                cj
            );
            crate::log::log_err(&msg);
            return "".to_string();
        },
    };

    return json["choices"][0]["text"].as_str().unwrap().into();
}

pub fn get_models_from_json(mj: &String) -> Vec<GPTModel> {
    let json: serde_json::Value = match serde_json::from_str(&mj) {
        Ok(val) => val,
        Err(e) => {
            let msg = format!(
                "\n\n{} [ERROR] {}\nWhile parsing response: {}\n\n", 
                crate::helpers::timestamp_full(), 
                e.to_string(), 
                mj
            );
            crate::log::log_err(&msg);
            return vec!();
        },
    };
    let mut result: Vec<GPTModel> = vec!();
    let mut count: i32 = 1;
    for mobj in json["data"].as_array().unwrap() {
        result.push(GPTModel {
            id: count,
            name: mobj["id"].as_str().unwrap().into(),
            owned_by: mobj["owned_by"].as_str().unwrap().into(),
        });
        count += 1;
    }

    return result;
}