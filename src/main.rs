mod model;
mod network;
mod utils;

use crate::model::config::Config;
use crate::utils::verbose::{log_verbose, set_verbose};
use clap::Parser;
use colored::Colorize;
use model::openrouter::response;
use network::api::Api;
use regex::Regex;
use std::io;
use std::io::Write;
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use utils::loader::{clear_loader, loader};

const DESTRUCTIVE_COMMANDS: &[&str] = &[
    "rm",
    "rmdir",
    "mkfs",
    "dd",
    "shred",
    "wipefs",
    "fdisk",
    "parted",
    "kill",
    "pkill",
    "killall",
    "chmod",
    "chown",
    "truncate",
    "mv",
    "/dev/sd",
    "-rf",
    "-fr",
    "-r",
    "-f",
    "if=",
    ":(){:|:&};:", // fork bomb
    "777",
    "mv /",
];

/// AI tool that converts your promt to CLI commands.
/// You can use any openai-like API (set URL with -e and api-key with -a if needed)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Promt to process
    #[arg(trailing_var_arg = true)]
    promt: Vec<String>,

    /// set openrouter api key
    #[arg(short, long)]
    api_key: Option<String>,

    /// list openrouter models
    #[arg(short, long)]
    list_models: bool,

    /// verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// set openrouter model id
    #[arg(short, long)]
    model: Option<String>,

    /// set openai-like api endpoint
    #[arg(short, long)]
    endpoint: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut work_done = false;

    set_verbose(args.verbose);

    if let Some(api_key) = args.api_key {
        match Config::set_api_key(&api_key) {
            Ok(_) => println!("Api key saved"),
            Err(e) => println!("Error saving: {e}"),
        }

        work_done = true;
    }

    if let Some(endpoint) = args.endpoint {
        let re = Regex::new(r#"https?://[^\s<>"{}|\\^`\[\]]+"#).unwrap();

        if re.is_match(&endpoint) {
            Config::set_endpoint(&endpoint).unwrap();
            println!("New endpoint: {endpoint}");
        } else {
            println!("Invalid url");
            return;
        }

        work_done = true;
    }

    if let Some(model) = args.model {
        let models = get_models().await.data;

        if models.contains(&response::Model { id: model.clone() }) {
            match Config::set_model(&model) {
                Ok(_) => println!("Model selected"),
                Err(e) => println!("Error selecting: {e}"),
            }
        } else {
            println!("No such model");
            return;
        }

        work_done = true;
    }

    if args.list_models {
        let model_id = if let Ok(config) = Config::read_config() {
            config.model_id
        } else {
            String::new()
        };

        if model_id.is_empty() {
            println!("No selected model");
        }

        get_models().await.data.iter().for_each(|model| {
            if model.id == model_id {
                println!("{} (selected)", model.id.green())
            } else {
                println!("{}", model.id)
            }
        });

        return;
    }

    if !args.promt.is_empty() {
        let promt = args.promt.join(" ");
        let command = &make_request(&promt).await.choices[0].message.content;
        print_command(command);
        print!("Accept? ({}/{}): ", "y".green(), "n".red());
        io::stdout().flush().unwrap();

        let answer = read_line();

        if answer == "y" {
            let output = Command::new("sh").arg("-c").arg(command).output().unwrap();

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    } else if !work_done {
        println!("No promt provided. Usage: cliai <promt>");

        return;
    };
}

async fn make_request(promt: &str) -> response::Payload {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let loader_thread = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            loader()
        }
    });

    let response = with_config(|api| async move { api.completions(promt).await }).await;

    running.store(false, Ordering::Relaxed);
    loader_thread.join().unwrap();

    clear_loader();

    response
}

async fn get_models() -> response::Models {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let loader_thread = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            loader()
        }
    });

    let response = with_config(|api| async move { api.models().await }).await;

    running.store(false, Ordering::Relaxed);
    loader_thread.join().unwrap();

    clear_loader();

    response
}

async fn with_config<T, F, Fut>(block: F) -> T
where
    F: FnOnce(Api) -> Fut,
    Fut: Future<Output=T>,
{
    match Config::read_config() {
        Ok(config) => {
            let api = Api::new(&config.api_key, &config.model_id, &config.endpoint);
            block(api).await
        }
        Err(e) => {
            eprintln!("Error reading config: {e}");
            std::process::exit(1);
        }
    }
}

fn print_command(command: &str) {
    log_verbose(command);

    let (safety, command) = {
        let lines: Vec<&str> = command.lines().collect();
        if lines.len() < 2 {
            (None, command.to_string())
        } else {
            (
                Some(lines[0]),
                lines[1..].join("\n"),
            )
        }
    };

    let danger = {
        match safety {
            None => { false }
            Some(safety) => { if safety.trim().to_lowercase() == "danger" { true } else { false } }
        }
    };

    let frame = "─".repeat(command.len() + 2);

    let command: String = if danger {
        command.red().to_string()
    } else {
        let s_array = command.split(" ");

        if DESTRUCTIVE_COMMANDS.contains(&&*command.replace(" ", "")) {
            command.red().to_string()
        } else {
            s_array
                .map(|part| {
                    if DESTRUCTIVE_COMMANDS.contains(&&*part.to_lowercase()) {
                        part.red()
                    } else {
                        part.yellow()
                    }
                })
                .map(|part| format!("{part} "))
                .collect::<String>()
        }
    };

    println!("┌{}┐", frame);
    println!("│ {} │", command.trim());
    println!("└{}┘", frame);
}

fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}
