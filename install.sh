#!/bin/bash

# Collection of build, install, run commands
# requires cargo and rustc

# builds the binary to /target/release/rust_gpt_cli
cargo build --release

# Install in specific location (optional)
# cp /target/release/rust_gpt_cli /opt/rust_gpt_cli/rust_gpt_cli

# Set ENV variables before running 
# export OPENAI_API_KEY="key" # (mandatory)
# export OPENAI_MAX_TOKENS="1000" # (optional)
# export RUST_GPT_CLI_THEME="base16-eighties.dark" # (optional)
# CHATGPT_SYSTEM_MSG is used exclusively by GPT models (optional)
# export CHATGPT_SYSTEM_MSG="You are ChatGPT, a large language model trained by OpenAI. Answer as concisely as possible."

# Run with cargo
cargo run --release

# Run binary
# ./target/release/rust_gpt_cli
