use chrono::Local;

pub fn get_timestamp() -> String {
    return Local::now().format("[%d/%m/%y %H:%M:%S]").to_string();
}

pub fn code_block_patterns() -> Vec<String> {
    return vec![
        "'''".into(), 
        "<code>".into(), 
        "</code>".into(),
    ];
}

pub fn sanitize_json(raw_stream_data: &String) -> Vec<String> {
    let mut sanitized = Vec::new();
    let mut rsd = raw_stream_data.clone();
    while rsd.contains("data: ") {
        rsd = rsd.replacen("data: ", "", 1);
        match parse_first_json(&rsd) {
            Some(obj) => {
                sanitized.push(obj.to_string());
                rsd = rsd[obj.len()..].to_string();
            },
            None => break,
        }
    }
    return sanitized
}

fn parse_first_json(input: &str) -> Option<&str> {
    let expected_end_of_json = "}], \"model\": \"text-davinci-002\"}";
    let end = input.find(expected_end_of_json)? + expected_end_of_json.len() + 1;

    Some(&input[..end])
}