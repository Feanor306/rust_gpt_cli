use serde::Serialize;
use crate::{env, helpers, syntax::SyntaxHighlighter};

const ROLE_SYSTEM: &str = "system";
const ROLE_USER: &str = "user";
const ROLE_ASSISTANT: &str = "assistant";
const RIGHT_GUTTER: usize = 1;

#[derive(Debug)]
pub struct GPTModel {
    pub id: i32,
    pub name: String,
    pub owned_by: String,
}

#[derive(Debug, Clone)]
pub struct RequestParams {
    pub prompt: String,
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32,
    pub messages: ChatMessages,
}

#[derive(Debug, Serialize)]
pub struct RPCompletions {
    pub prompt: String,
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32,
}

#[derive(Debug, Serialize)]
pub struct RPChat {
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32,
    pub messages: Vec<ChatMessage>,
}

impl RequestParams {
    pub fn new(sm: &String, mt: i32) -> Self {
        Self {
            prompt: "".into(),
            model: env::DEFAULT_MODEL.into(),
            stream: true,
            max_tokens: mt,
            messages: ChatMessages::new(sm),
        }
    }

    pub fn to_completion(&self) -> RPCompletions {
        RPCompletions {
            prompt: self.prompt.clone(),
            model: self.model.clone(),
            stream: self.stream,
            max_tokens: self.max_tokens,
        }
    }

    pub fn to_chat(&self) -> RPChat {
        RPChat {
            messages: self.messages.messages.clone(),
            model: self.model.clone(),
            stream: self.stream,
            max_tokens: self.max_tokens,
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

        // syntax highlighting when line reaches width of screen
        if self.last_line.len() + RIGHT_GUTTER == (self.max_width as usize) {
            println!("");
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

#[derive(Debug, Serialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatMessages {
    pub messages: Vec<ChatMessage>,
    pub total_tokens: i32,
}

impl ChatMessages {
    pub fn new(s: &String) -> Self {
        let mut m = vec![];
        if !s.is_empty() {
            m.push(ChatMessage {
                role: ROLE_SYSTEM.into(),
                content: s.into(),
            });
        }
        Self {
            messages: m,
            total_tokens: 0,
        }
    }

    pub fn get_last_message(&mut self) -> String {
        match self.messages.last() {
            Some(c) => return c.content.clone(),
            None => return "".into(),
        }
    }

    pub fn add_user_msg(&mut self, s: &String) {
        self.messages.push(ChatMessage {
            role: ROLE_USER.into(),
            content: s.into(),
        });
    }

    pub fn add_assistant_msg(&mut self, s: &String) {
        self.messages.push(ChatMessage {
            role: ROLE_ASSISTANT.into(),
            content: s.into(),
        });
    }
}
