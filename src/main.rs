use clap::Parser;
use reqwest::{header, Client};
use serde::Deserialize;
use std::{
    fs,
    io::{self, Read},
};
use sysinfo::{System, SystemExt};
use chrono::Local;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    prompt: Option<String>,

    #[clap(long, default_value = "gpt-4.1-mini")]
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

fn load_config() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("Home directory not found.")?;
    let config_path = home_dir.join(".heystupid.config");
    if !config_path.exists() {
        return Err(format!(
            "Configuration file {} not found. Please create this file with the necessary configurations.",
            config_path.display()
        ));
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file {}: {}", config_path.display(), e))?;

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix("openai_api_key") {
            if let Some((_, key)) = stripped.split_once('=') {
                let key = key.trim();
                if !key.is_empty() {
                    return Ok(key.to_string());
                }
            }
        }
    }

    Err(format!(
        "openai_api_key not found or empty in config file {}",
        config_path.display()
    ))
}

fn base_prompt() -> &'static str {
    "This is a command line tool that accepts command output and a user prompt.
Responses should be concise and formatted to wrap at 80 characters long.
Do not include formatting characters or markdown. Multi-line output is acceptable.
Avoid praise and filler text. Respond with summations or evaluations of errors to help the user."
}

fn is_stdin_tty() -> bool {
    atty::is(atty::Stream::Stdin)
}

fn system_stats() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let datetime = Local::now().to_rfc3339();
    let os = sys.name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let cpu_count = sys.cpus().len();
    let total_memory_mb = sys.total_memory() / 1024;
    let available_memory_mb = sys.available_memory() / 1024;
    let uptime_sec = sys.uptime();

    format!(
        r#"{{"datetime":"{}","os":"{}","os_release":"{}","hostname":"{}","cpu_count":{},"memory_total_mb":{},"memory_available_mb":{},"uptime_sec":{}}}"#,
        datetime, os, os_version, hostname, cpu_count, total_memory_mb, available_memory_mb, uptime_sec
    )
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let open_ai_key = load_config()?;

    let args = Args::parse();

    let mut stdin_input = String::new();
    if !is_stdin_tty() {
        io::stdin()
            .read_to_string(&mut stdin_input)
            .map_err(|_| "Failed to read from stdin.".to_string())?;
    }

    let prompt = if let Some(prompt) = args.prompt {
        if stdin_input.trim().is_empty() {
            prompt
        } else {
            format!("{} {}", stdin_input.trim(), prompt)
        }
    } else {
        stdin_input.trim().to_string()
    };

    if prompt.is_empty() {
        eprintln!("Error: No input provided.\nUsage examples:\n  heystupid 'What is Rust?'\n  echo 'text' | heystupid 'Explain this'\n  ls /etc/ | heystupid 'What OS is this?'");
        std::process::exit(1);
    }

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let messages = vec![
        serde_json::json!({
            "role": "system",
            "content": system_stats()
        }),
        serde_json::json!({
            "role": "system",
            "content": base_prompt()
        }),
        serde_json::json!({
            "role": "user",
            "content": prompt
        }),
    ];

    let response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", open_ai_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "model": args.model,
            "messages": messages
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