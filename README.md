[rust_gpt_cli] is a CLI tool that communicates with OpenAI API, using one of their models. It creates a stream for each prompt and streams the response back to the terminal asynchronously. Can be used with ChatGPT models.

# Features
* Streaming GPT responses asynchronously in the terminal
* Syntax highlighting for code (determined by programming language if present in prompt)
* Syntax highlighting themes can be set via **RUST_GPT_CLI_THEME** env var
* Change model by querying **/models** API and choosing one of the available models (default text-davinci-003)
* ChatGPT model support by calling **/chat/completions**. The main difference to other models is that each request contains the whole chat history of the session (with the same model), cleared on terminating the program or changing the model. This can have implications for token restrictions as the token count of each consecutive request increases cumulatively.

# Installation
Refer to **install.sh** for all installation info  
Most notably
```bash
# builds the binary to /target/release/rust_gpt_cli
cargo build --release

# Run with cargo
cargo run --release

# Or run binary
# ./target/release/rust_gpt_cli
```

# ENV Variables
The program uses the following environment variables.
```bash
# OpenAI API KEY. 
# MANDATORY
OPENAI_API_KEY="key"

# Max number of tokens the model will use. 
# Includes Prompt Length + Response Length
# 1 token is approximately 4 characters or 0.75 words
OPENAI_MAX_TOKENS="1000"

# System message at the beginning of a chat
# optional and used only by ChatGPT models like gpt-3.5-turbo and gpt-4
CHATGPT_SYSTEM_MSG="You are ChatGPT, a large language model trained by OpenAI. Answer as concisely as possible."

# Syntax Highlighting theme
# https://docs.rs/syntect/latest/syntect/highlighting/struct.ThemeSet.html
RUST_GPT_CLI_THEME="base16-eighties.dark"
```

# Tested models
* text-davinci-003
* text-davinci-002
* code-davinci-002
* gpt-3.5-turbo
