[rust_gpt_cli] is a CLI tool that communicates with OpenAI GPT model. It creates a stream for each prompt and streams the response back to the terminal asynchroniously. 

# Features
* Streaming GPT responses asynchroniously in the terminal
* Syntax highlighting for code (determined by programming language if present in prompt)
* Change model by querying **/models** API and choosing one of the available models (default text-davinci-003)

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

# Set Properties
The program uses the following environment variables.
```bash
# OpenAI API KEY. MANDATORY.
OPENAI_API_KEY="key"

# Max number of tokens the model will use. 
# Includes Prompt Length + Response Length
# 1 token is approximately 4 characters or 0.75 words
OPENAI_MAX_TOKENS="200"
```

# Tested models
* text-davinci-003
* text-davinci-002

# Known issues
* handle rows that are wider than terminal?
