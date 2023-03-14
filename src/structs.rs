use std::io::stdout;
use serde::{Deserialize, Serialize};
use crossterm::{cursor, ExecutableCommand};
use crate::{helpers, syntax::SyntaxHighlighter};

#[derive(Debug)]
pub struct GPTModel {
    pub id: i32,
    pub name: String,
    pub owned_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestParams {
    pub prompt: String,
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32,
}

impl RequestParams {
    pub fn new(mt: i32) -> Self {
        Self {
            prompt: "".into(),
            model: "text-davinci-003".into(),
            stream: true,
            max_tokens: mt,
        }
    }
}

#[derive(Debug)]
pub struct GPTResponse {
    pub prompt: String,
    pub full_response: String,
    pub last_line: String,
    pub max_width: u16,
    pub syntax: SyntaxHighlighter,
}

impl GPTResponse {
    pub fn new(p: &String) -> Self {
        Self {
            prompt: p.into(),
            full_response: "".into(),
            last_line: "".into(),
            max_width: helpers::get_screen_width(),
            syntax: SyntaxHighlighter::new(&p),
        }
    }
    pub fn append_full(&mut self, chunk: String) {
        self.full_response.push_str(&chunk);
    }
    pub fn append_line(&mut self, chunk: String) {
        self.last_line.push_str(&chunk);
        if self.last_line.len() == (self.max_width as usize) {
            let mut stdout = stdout();
            stdout.execute(cursor::MoveDown(1)).unwrap();
            stdout.execute(cursor::MoveToColumn(0)).unwrap();
            self.reset_line();
        }
    }
    pub fn reset_line(&mut self) {
        // syntax highliting for previous line every time a newline is streamed back in response
        if !self.last_line.is_empty() {
            self.syntax.reprint_with_style(&self.last_line);
            self.last_line = "".into();
        }
    }
}
