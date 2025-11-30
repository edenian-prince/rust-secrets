use colored::Colorize;
use regex::Regex;
use std::path::Path;
use std::process::Command;

use crate::utils::*;
pub mod utils;

pub fn pre_commit_hook_scan(custom_patterns: Option<Regex>) {
    // Read in the global regex - this will fail if it's is_empty
    let global_secrets = read_patterns();

    let mut patterns = Vec::new();
    patterns.extend(global_secrets);

    if let Some(re) = custom_patterns {
        patterns.push(re);
    }

    // now we need to flip through the staged git files and search for regex matches
    let staged_files = get_staged_files();

    let mut secrets_found = false;

    for file in &staged_files {
        if let Some(content) = get_staged_content(file) {
            for re in patterns.iter() {
                for (line_number, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        println!(
                            "{} {} {} {} {}\n",
                            "Pattern #".blue(),
                            re.to_string().yellow(),
                            "matched in".blue(),
                            file.to_string().magenta(),
                            format!("at line {}: {}", line_number + 1, line.red()).cyan()
                        );
                        secrets_found = true;
                    }
                }
            }
        } else {
            eprintln!("Could not read staged content for {}", file)
        }
    }

    if secrets_found {
        eprintln!(
            "{} {}\n",
            "Secret scan failed. Commit aborted.".red().bold(),
            "use `--no-verify` if this was a false positive"
                .red()
                .bold()
        );
        std::process::exit(1);
    } else {
        println!("{}", "No secrets found.".green().bold());
        std::process::exit(0);
    }
}

pub fn scan(custom_patterns: Option<Regex>) {
    // Read in the global regex - this will fail if it's is_empty
    let global_secrets = read_patterns();

    let mut patterns = Vec::new();
    patterns.extend(global_secrets);

    if let Some(re) = custom_patterns {
        patterns.push(re);
    }

    for (i, re) in patterns.iter().enumerate() {
        println!(
            "{} {}",
            "Scanning for regex:".blue().bold(),
            re.as_str().cyan()
        );

        let output = Command::new("git")
            .args(["log", "-G", re.as_str(), "--oneline"])
            .output()
            .expect("Failed to run git log on regex.");

        if !output.status.success() {
            eprint!(
                "Git command failed for pattern #{}: {}",
                i + 1,
                String::from_utf8_lossy(&output.stderr)
            );
            continue;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim().is_empty() {
            println!("No matches found for pattern #{}\n", i + 1);
        } else {
            println!(
                "Match found for pattern #{}. For more info, search the following hash numbers in GitHub:\n{}",
                i + 1,
                stdout
            );
        }
    }
}

// add provider lists
pub fn add_config(path: &str, local: bool) {
    let file = Path::new(path);
    let is_path = file.is_file();
    let mut private = false;

    if is_path {
        println!("this is a local file path");
        println!(
            "if it's a git repo, would you like to automatically pull updates from this provider txt file? y/n"
        );
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        // assign to private
        private = if input == "y" || input == "yes" {
            true
        } else if input == "n" || input == "no" {
            false
        } else {
            println!("Invalid input.");
            return;
        };
    }

    let key = if private {
        "git-find.private-file"
    } else {
        "git-find.regex-file"
    };

    // get all existing values
    let output = Command::new("git")
        .args(["config", "--get-all", key])
        .output()
        .expect("failed to run git config");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let exists = stdout.lines().any(|line| line.trim() == path);

    if exists {
        println!("key already exists");
    } else {
        // logic for local path
        if local {
            println!("adding path to local");
            Command::new("git")
                .args(["config", "--add", key, path])
                .output()
                .expect("failed to add a key.");
        } else {
            println!("adding key path to global git config");
            Command::new("git")
                .args(["config", "--global", "--add", key, path])
                .output()
                .expect("failed to add a key.");
        }
    }
}

pub fn list_providers() {
    let gc = Command::new("git")
        .args(["config", "--global", "--get-regex", "git-find.*"])
        .output()
        .expect("listing providers failed.. do you have providers set up?");

    println!("{}", String::from_utf8_lossy(&gc.stdout));
}
