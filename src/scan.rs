use colored::Colorize;
use regex::Regex;
use std::process::Command;

use crate::utils::*;

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
