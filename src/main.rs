use clap::{Parser, Subcommand};
use regex::Regex;
mod hook;
mod install;
mod providers;
mod scan;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// List secrets providers in your git config
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    list: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// scans for regex in global config
    Hook {},
    /// install the hooks
    Install {},
    /// scan the repo for regexs in providers list or custom regex
    Scan {
        #[arg(short, long)]
        custom_patterns: Option<String>,
    },
    /// add a regex file to global git config
    AddProvider {
        /// git config paths
        #[arg(short, long)]
        path: String,
        /// optional - add local provider
        #[arg(short, long, default_value_t = false)]
        local: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // --list flag to show global providers
    if cli.list {
        // runs git config --global --get-regex git-find.*
        providers::list_providers();
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Hook {}) => {
            hook::pre_commit_hook_scan(None);
        }
        Some(Commands::Install {}) => {
            install::install_hooks();
        }
        Some(Commands::Scan { custom_patterns }) => {
            if let Some(pattern_str) = custom_patterns {
                // Convert the string into a Regex safely
                match Regex::new(pattern_str) {
                    Ok(user_regex) => scan::scan(Some(user_regex)),
                    Err(e) => eprintln!("Invalid regex: {}", e),
                }
            } else {
                scan::scan(None); // No regex provided
            }
        }
        Some(Commands::AddProvider { path, local }) => {
            providers::add_config(path, *local);
        }
        None => {}
    }

    // Continued program logic goes here...
}
