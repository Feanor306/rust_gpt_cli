use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestParams {
    pub prompt: String,
    pub model: String,
    pub stream: bool,
    pub max_tokens: i32
}

#[derive(Debug, PartialEq)]
pub enum OutputState {
    Default,
    CodeBlock
}

#[derive(Debug)]
pub struct GPTResponse {
    pub payload: String,
    pub state: OutputState,
}

impl GPTResponse {
    pub fn new() -> Self {
        Self {
            payload: "".into(),
            state: OutputState::Default,
        }
    }
    pub fn append(&mut self, chunk: String) {
        self.payload.push_str(&chunk);
        self.check_code_block();
    }
    pub fn check_code_block(&mut self) {
        for p in crate::helpers::code_block_patterns() {
            if self.payload.contains(&p) {
                self.payload = self.payload.replace(&p, "");
                if self.state == OutputState::Default {
                    self.state = OutputState::CodeBlock;
                } else {
                    self.state = OutputState::Default;
                }
            }
        }
    }
}