use tempfile::TempDir;
use std::fs::metadata;
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::read_to_string;
// use std::fs::OpenOptions;
use colored::Colorize;
use regex::Regex;
use std::process::Command;

use crate::utils::*;

pub fn collect_patterns(custom_patterns: Option<Regex>) -> Vec<Regex> {
    // Read in the global regex - this will fail if it's is_empty
    let global_secrets = read_patterns();

    let mut patterns = Vec::new();

    patterns.extend(global_secrets);
    if let Some(re) = custom_patterns {
        patterns.push(re);
    }

    patterns

}

pub fn scan(custom_patterns: Option<Regex>) {
    let patterns = collect_patterns(custom_patterns);

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

pub fn scan_org(repo_list: &str, custom_patterns: Option<Regex>){
    // let list_repos = Command::new("gh")
    //     .args([
    //         "repo",
    //         "list",
    //         org_name,
    //         "--limit",
    //         "1000",
    //         "--json",
    //         "nameWithOwner",
    //         "--jq",
    //         ".[].nameWithOwner",
    //     ])
    //     .output()
    //     .expect("gh cli pull for repos in org failed.")
    //  instead of doing the gh command for the user,
    //  make the user input their repos in format org/repo
    //  that way they can do this for however many repos they want 
    //  and this code doesn't need to account for api limits 
    
    // let stdout = String::from_utf8_lossy(&list_repos.stdout);
    // let mut csv = OpenOptions:: new()
    //     .create(true)
    //     .append(true)
    //     .open("results.csv")
    //     .expect("cannot open csv");
    let mut csv = File::create("results.csv")
        .expect("cannot create csv");

    if metadata("results.csv").map(|m| m.len() == 0).unwrap_or(true) {
        writeln!(csv, "repo,commit,pattern").expect("header write failed");
    }

    let contents = read_to_string(repo_list)
        .expect("failed to read repo list");

    let patterns = collect_patterns(custom_patterns);

    let dir = TempDir::new().expect("temp dir");
    let base_path = dir.path();

    let total = contents.lines().count() as u64;

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}"
        )
        .unwrap()
        .progress_chars("##-"),
    );

    for line in contents.lines(){
        pb.set_message(format!("Scanning {}", line));

        //let repo_name = line.split('/').last().unwrap();
        // let github_url = format!("https://github.com/{line}.git");
        let repo_dir_name = line.replace("/", "_"); // safe folder name
        let repo_path = base_path.join(&repo_dir_name);
        
        let git_clone = Command::new("gh")
            .args(["repo","clone", line, repo_path.to_str().unwrap()])
            .output()
            .expect("could not clone repo.");

        if !git_clone.status.success() {
            eprintln!("Failed to clone {}", line);
            continue;
        }
        //let repo_path = PathBuf::from(repo_name);
        //let dir = TempDir::new().expect("temp dir");
        //let repo_path = dir.path().join(repo_name);

        for (i, re) in patterns.iter().enumerate() {
            // println!(
            //     "{} {}",
            //     "Scanning for regex:".blue().bold(),
            //     re.as_str().cyan()
            // );

            let git_log_output = Command::new("git")
                .args(["log", "-G", re.as_str(), "--pretty=format:%h"])
                .current_dir(&repo_path)
                .output()
                .expect("Failed to run git log on regex.");

            if !git_log_output.status.success() {
                eprint!(
                    "Git command failed for pattern #{}: {}",
                    i + 1,
                    String::from_utf8_lossy(&git_log_output.stderr)
                );
                continue;
            }

            let commits = String::from_utf8_lossy(&git_log_output.stdout);

            for commit in commits.lines() {
                writeln!(
                    csv,
                    "{},{},{}",
                    repo_dir_name,
                    commit,
                    re.as_str()
                )
                .expect("write failed.");
            }
        }
        pb.inc(1);
    }

}


