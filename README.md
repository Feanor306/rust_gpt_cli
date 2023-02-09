[rust_gpt_cli] is a CLI tool that communicates with OpenAI GPT model. It creates a stream for each prompt and streams the response back to the terminal asynchroniously. 

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

TODO Features:
1. Option to switch model (query endpoint models?)
2. Option to query a different endpoint
