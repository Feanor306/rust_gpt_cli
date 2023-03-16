use std::env;
use std::cmp::min;

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
    // Default 1000 ~> 750 words
    let default_max_tokens: i32 = 1000;
    // Max_max 4000 -> 3750 words
    let max_max_tokens: i32 = 4000; 

    let mts = match env::var("OPENAI_MAX_TOKENS") {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to read environment variable: {}", e);
            "0".into()
        }
    };

    let mt: i32 = mts.parse().unwrap();

    if mt > 0 {
        return min(mt, max_max_tokens);
    } else {
        return default_max_tokens;
    }
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
    let default_theme: String = "base16-eighties.dark".into();

    let theme = match env::var("RUST_GPT_CLI_THEME") {
        Ok(val) => val,
        Err(_) => default_theme.clone(),
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
        return default_theme.clone()
    }
}
