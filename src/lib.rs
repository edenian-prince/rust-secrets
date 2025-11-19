use colored::Colorize;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::process::Command;
use std::{env, fs};

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

pub fn install_hooks() {
    // pull a remote secrets regex path and put it in
    // global gitconfig

    let mut git_template = env::home_dir().expect("Could not find home directory");
    git_template.push(".git-template/hooks");

    if let Err(e) = fs::create_dir_all(&git_template) {
        eprintln!("Failed to create hooks directory: {}", e);
        return;
    }

    git_template.push("pre-commit");

    if git_template.is_file() {
        println!("pre-commit hook exists; check for existing git-find command");

        let hook_line = "git find hook";

        // Read the existing file contents
        let contents = match fs::read_to_string(&git_template) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                return;
            }
        };

        // Only append if the line doesn't already exist
        if contents.contains(hook_line) {
            println!("Hook already exists, skipping append");
        } else {
            println!("Adding hook to file");

            let file_result = OpenOptions::new()
                .append(true)
                .create(true)
                .open(git_template);

            let file = match file_result {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    return;
                }
            };

            let mut writer = BufWriter::new(file);
            writeln!(writer, "\ngit find hook").unwrap();
        }
    } else {
        println!("pre-commit hook file not found... creating file");

        match fs::File::create(&git_template) {
            Ok(file) => {
                let mut writer = BufWriter::new(file);
                writeln!(writer, "#!/bin/sh").unwrap();
                writeln!(writer, "# Hook installed by Rust script").unwrap();
                writeln!(writer, "git find hook").unwrap();
                println!("Created new pre-commit hook at {:?}", git_template);
            }
            Err(e) => eprintln!("Failed to create pre-commit hook: {}", e),
        }
    }

    let home_dir = env::home_dir().expect("Could not find home directory");

    let template_dir = home_dir.join(".git-template");
    let hooks_dir = template_dir.join("hooks");
    let pc_dir = hooks_dir.join("pre-commit");

    Command::new("git")
        .args([
            "config",
            "--global",
            "init.templateDir",
            template_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to set global git template directory");

    // need to configure the git global hooks path
    Command::new("git")
        .args([
            "config",
            "--global",
            "core.hooksPath",
            hooks_dir.to_str().unwrap(),
        ])
        .status()
        .expect("failed to set up global hooks path");

    // need to make it executable!
    Command::new("chmod")
        .args(["+x", pc_dir.to_str().unwrap()])
        .status()
        .expect("failed to make hooks path executable");
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
