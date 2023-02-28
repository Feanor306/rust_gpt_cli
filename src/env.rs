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
