# Slang Translator

This is a simple web application built with Rust using the Actix-web framework. It translates slang into charming, Atlanta-flavored English using AI models from OpenAI and Anthropic.

## Features

- Translate slang to more charming and rizz-infused English.
- Uses AI models from OpenAI and Anthropic.
- Simple web interface with Bootstrap styling.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

1. **Clone the repository:**

    ```sh
    git clone https://github.com/your-username/slang_translator.git
    cd slang_translator
    ```

2. **Set up environment variables:**

    Ensure you have your API keys and gateway URL set up as environment variables:

    ```sh
    export OPENAI_API_KEY="your_openai_api_key"
    export ANTHROPIC_API_KEY="your_anthropic_api_key"
    export CLOUDFLARE_AI_GATEWAY_URL="your_cloudflare_ai_gateway_url"
    export PORT=5000  # Optional, default is 5000
    ```

3. **Build and run the application:**

    ```sh
    cargo run
    ```

## Usage

Open your web browser and navigate to `http://localhost:5000`. You should see the Slang Translator web interface.

## Project Structure

- `src/main.rs`: The main application code.
- `templates/home.html`: The HTML template for the web interface.
- `static/robots.txt`: The robots.txt file.
- `static/ads.txt`: The ads.txt file.
- `Cargo.toml`: The Cargo configuration file.

## Dependencies

This project uses the following Rust crates:

- `actix-web`: Web framework for Rust.
- `actix-files`: Static file serving for Actix-web.
- `askama`: Templating engine for Rust.
- `serde`: Serialization and deserialization library.
- `serde_json`: JSON support for Serde.
- `reqwest`: HTTP client for Rust.
- `env_logger`: Logger for Rust applications.
- `log`: Logging facade for Rust.
- `tokio`: Asynchronous runtime for Rust.

## Acknowledgements

- [Actix-web](https://actix.rs/) for the web framework.
- [Askama](https://github.com/djc/askama) for the templating engine.
- [OpenAI](https://www.openai.com/) and [Anthropic](https://www.anthropic.com/) for their AI models.
