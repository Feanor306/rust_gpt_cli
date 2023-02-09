use serde::{Deserialize, Serialize};
use crate::syntax::SyntaxHighlighter;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestParams {
    pub prompt: String,
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32
}

#[derive(Debug)]
pub struct GPTResponse {
    pub prompt: String,
    pub full_response: String,
    pub last_line: String,
    pub syntax: SyntaxHighlighter,
}

impl GPTResponse {
    pub fn new(p: &String) -> Self {
        Self {
            prompt: p.into(),
            full_response: "".into(),
            last_line: "".into(),
            syntax: SyntaxHighlighter::new(&p),
        }
    }
    pub fn append_full(&mut self, chunk: String) {
        self.full_response.push_str(&chunk);
    }
    pub fn append_line(&mut self, chunk: String) {
        self.last_line.push_str(&chunk);
    }
    pub fn reset_line(&mut self) {
        // syntax highliting for previous line every time a newline is streamed back in response
        self.syntax.reprint_with_style(&self.last_line);
        self.last_line = "".into();
    }
}