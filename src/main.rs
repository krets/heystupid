use clap::Parser;
use dotenv::from_path;
use reqwest::header;
use serde::Deserialize;
use std::env;
use std::io::{self, Read};
use std::path::Path;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The prompt question to ask OpenAI
    prompt: Option<String>,

    /// OpenAI model to use
    #[clap(long, default_value = "gpt-4o-mini")]
    model: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

fn setup_environment() -> Result<(), Box<dyn std::error::Error>> {
    // Get home directory
    let home = env::var("HOME").map_err(|_| {
        "HOME environment variable not set. Please ensure you're running on a Unix-like system."
    })?;
    
    let dotenv_path = Path::new(&home).join(".heystupid");
    
    // Check if .heystupid file exists
    if !dotenv_path.exists() {
        return Err(format!(
            "Configuration file not found: {}\n\
            Please create this file with your OpenAI API key:\n\
            echo 'OPENAI_API_KEY=your_api_key_here' > {}",
            dotenv_path.display(),
            dotenv_path.display()
        ).into());
    }
    
    // Load environment variables from the specified file
    from_path(&dotenv_path).map_err(|e| {
        format!(
            "Failed to load configuration from {}: {}\n\
            Please ensure the file exists and contains: OPENAI_API_KEY=your_api_key_here",
            dotenv_path.display(),
            e
        )
    })?;
    
    // Verify the API key is set
    env::var("OPENAI_API_KEY").map_err(|_| {
        format!(
            "OPENAI_API_KEY not found in {}.\n\
            Please add your OpenAI API key to the file:\n\
            echo 'OPENAI_API_KEY=your_api_key_here' >> {}",
            dotenv_path.display(),
            dotenv_path.display()
        )
    })?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup environment with better error handling
    if let Err(e) = setup_environment() {
        eprintln!("Setup Error: {}", e);
        std::process::exit(1);
    }

    let args = Args::parse();

    // Read from stdin if present
    let mut stdin_input = String::new();
    if atty::isnt(atty::Stream::Stdin) {
        io::stdin().read_to_string(&mut stdin_input)?;
    }

    // Combine stdin input and prompt
    let prompt = match args.prompt {
        Some(p) => {
            if stdin_input.trim().is_empty() {
                p
            } else {
                format!("{} {}", stdin_input.trim(), p)
            }
        }
        None => stdin_input.trim().to_string(),
    };

    if prompt.is_empty() {
        eprintln!("Error: No input provided.");
        eprintln!("Usage examples:");
        eprintln!("  heystupid 'What is Rust?'");
        eprintln!("  echo 'some text' | heystupid 'Explain this'");
        eprintln!("  ls /etc/ | heystupid 'What OS is this?'");
        std::process::exit(1);
    }

    let open_ai_key = env::var("OPENAI_API_KEY")?; // This should now be safe

    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let response = client.post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", open_ai_key))
        .json(&serde_json::json!({
            "model": args.model,
            "messages": [
                {
                    "role": "system",
                    "content": "This is a linux command line tool. The user is requesting concise responses. You should respond with exact answers only. The output system does not support any special formatting, so refrain from using extraneous characters. The prompt may be the output and error from another command piped into this tool. Respond with summations or evaluations of errors to help the user."
                },
                {"role": "user", "content": prompt}
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request to OpenAI: {}", e))?
        .json::<OpenAIResponse>()
        .await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

    if let Some(choice) = response.choices.first() {
        println!("{}", choice.message.content);
    } else {
        eprintln!("No response received from OpenAI");
        std::process::exit(1);
    }

    Ok(())
}
