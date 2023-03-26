use std::env;
use std::cmp::min;

pub const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const DEFAULT_THEME: &str = "base16-eighties.dark";

// Default 1000 ~> 750 words
const DEFAULT_MAX_TOKENS: i32 = 1000;
// Max_max 32000 ~> 24000 words
const MAX_MAX_TOKENS: i32 = 32000;

// Retrieves ENV VAR 
// OPENAI_API_KEY
pub fn get_api_key() -> String {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to read environment variable OPENAI_API_KEY: {}", e);
            return "".into();
        }
    };
    return api_key;
}

// Retrieves ENV VAR 
// OPENAI_MAX_TOKENS
// Returns Default value otherwise
// 1 token is approximately 4 characters or 0.75 words
pub fn get_max_tokens() -> i32 {
    let mts = match env::var("OPENAI_MAX_TOKENS") {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to read environment variable: {}", e);
            "0".into()
        }
    };

    let mt: i32 = mts.parse().unwrap();

    if mt > 0 {
        return min(mt, MAX_MAX_TOKENS);
    } else {
        return DEFAULT_MAX_TOKENS;
    }
}

// Retrieves ENV VAR 
// OPENAI_MODEL
// returns default value otherwise 
// currently default gpt-3.5-turbo
pub fn get_default_model() -> String {
    let model = match env::var("OPENAI_MODEL") {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to read environment variable OPENAI_MODEL: {}", e);
            return "".into();
        }
    };

    if model.len() == 0 {
        return DEFAULT_MODEL.into();
    }

    return model;
}

// Retrieves ENV VAR 
// CHATGPT_SYSTEM_MSG
pub fn get_system_message() -> String {
    let system_msg = match env::var("CHATGPT_SYSTEM_MSG") {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to read environment variable CHATGPT_SYSTEM_MSG: {}", e);
            return "".into();
        }
    };
    return system_msg;
}

// Retrieves ENV VAR 
// RUST_GPT_CLI_THEME
// Returns Default value otherwise
pub fn get_theme() -> String {
    let theme = match env::var("RUST_GPT_CLI_THEME") {
        Ok(val) => val,
        Err(_) => DEFAULT_THEME.into(),
    };

    let available_themes = vec![
        "base16-ocean.dark", 
        "base16-eighties.dark", 
        "base16-mocha.dark",
        "base16-ocean.light", 
        "InspiredGitHub",
        "Solarized (dark)", 
        "Solarized (light)",
    ];

    if available_themes.contains(&theme.as_str()) {
        return theme
    } else {
        return DEFAULT_THEME.into()
    }
}
