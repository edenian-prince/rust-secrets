use regex::Regex;
use std::error::Error;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{self, Path, PathBuf};
use std::process::Command;
use std::{env, fs};

// pub fn get_staged_files() -> Vec<String> {
//     let output = Command::new("git")
//         .args(["diff", "--cached", "--name-only"])
//         .output()
//         .expect("failed to list staged files");
//     String::from_utf8_lossy(&output.stdout)
//         .lines()
//         .map(|s: |s.to_string())
//         .collect()
// }
//
// Write config from github to .gitconfig
// it will clone the remote secrets repo and set up the inital .gitconfig
// to be used in the Install Command
// after initial install, we need a command that pulls just the single secrets file from the remote
// repo
pub fn write_git_regex_file(repo_url: &str, secrets_path: &str) {
    let tmp_dir = env::temp_dir().join("secrets_repo");
    if !tmp_dir.exists() {
        let _status = Command::new("git")
            .args(["clone", repo_url, tmp_dir.to_str().unwrap()])
            .output()
            .expect("unable to clone repo");
    } else {
        let _status = Command::new("git")
            .args(["pull", tmp_dir.to_str().unwrap()])
            .output()
            .expect("error in pulling the repo..");
    }

    let secrets_file = tmp_dir.join(secrets_path);
    if !secrets_file.exists() {
        eprintln!("Error! file doesn't exist")
    }

    let _write_to_config = Command::new("git")
        .args([
            "config",
            "--global",
            "git-find.regex-file",
            secrets_file.to_str().unwrap(),
        ])
        .output()
        .expect("failed to write to global config");
}

// Read in the .gitconfig and find the regex file(s), return a pathbuf
pub fn read_git_regex_files() {
    let output = Command::new("git")
        .args(["config", "--global", "--get", "git-find.regex-file"])
        .output()
        .expect("failed to find provider");

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let path = PathBuf::from(path_str);
    path;
}

pub fn load_regex_from_file(path: &str) -> std::io::Result<Vec<Regex>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let mut patterns = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?; // propagate read errors
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        match Regex::new(trimmed) {
            Ok(re) => patterns.push(re),
            Err(err) => eprintln!("invalid regex '{}': {}", trimmed, err),
        }
    }
    Ok(patterns)
}

pub fn pre_commit_hook_scan(custom_patterns: Option<Regex>) {
    let global_secret = read_git_regex_files();

    let mut patterns = Vec::new();

    if let Some(re) = custom_patterns {
        patterns.push(re);
    }
}
