use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

pub fn get_staged_files() -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .expect("failed to list staged files");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn get_staged_content(path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["show", &format!(":{}", path)])
        .output()
        .expect("problems getting staged files!");
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

// Read in the .gitconfig and find the regex file(s), return a pathbuf
pub fn load_file_patterns() -> Vec<PathBuf> {
    let output = Command::new("git")
        .args(["config", "--global", "--get-all", "git-find.regex-file"])
        .output()
        .expect("failed to find provider");

    // PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|line| PathBuf::from(line.trim()))
        .collect()
}

pub fn load_private_file_patterns() -> Vec<PathBuf> {
    // if private-file doesn't exist, just pass.
    let output = Command::new("git")
        .args(["config", "--global", "--get-all", "git-find.private-file"])
        .output()
        .expect("failed to find provider");

    // PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|line| PathBuf::from(line.trim()))
        .collect()
}

pub fn load_regex_from_file(path: &PathBuf) -> std::io::Result<Vec<Regex>> {
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

// pub fn load_private_repo_file(repo: &str) {
//     Command::new("git")
//         .args(["-C", &repo, "show", "origin/main", part])
//         .output()
//         .expect("couldn't pull private repo file");
// }
//
// for each file in the git config, open and read the file,
// and append to a regex list
// if the file is from github, curl the file and append
// this makes sure that the file can change remotely
pub fn read_patterns() -> Vec<regex::Regex> {
    let files = load_file_patterns();
    let mut patterns = Vec::new();

    for path in files {
        let path_str = &path.to_str().unwrap();

        if path_str.contains("github") {
            // Use a temporary file to store the downloaded content
            let tmp_path = "/tmp/github_regex.txt"; // or use tempfile crate for safer temp files

            let status = Command::new("curl")
                .args(["-s", "-L", "-o", tmp_path, path_str])
                .status()
                .expect("failed to run curl");

            if !status.success() {
                eprintln!("curl failed for {}", path_str);
            }

            // Now parse regexes from the downloaded file
            match load_regex_from_file(&PathBuf::from(tmp_path)) {
                Ok(regexes) => patterns.extend(regexes),
                Err(err) => eprintln!("Error reading downloaded file {}: {}", tmp_path, err),
            }
        } else {
            println!("Local file: {}", path_str);

            match load_regex_from_file(&path) {
                Ok(regexes) => {
                    println!("Loaded {} regexes", regexes.len());
                    patterns.extend(regexes);
                }
                Err(err) => {
                    println!("Error reading {}: {}", path_str, err);
                }
            }
        }
    }

    let private_files = load_private_file_patterns();
    if !private_files.is_empty() {
        for path in private_files {
            let path_str = &path.to_str().unwrap();

            let git_dir = path.parent().unwrap();

            let output = Command::new("git")
                .args([
                    "-C",
                    git_dir.to_str().unwrap(),
                    "rev-parse",
                    "--show-toplevel",
                ])
                .output()
                .expect("didnt work");

            let top_level = String::from_utf8_lossy(&output.stdout).trim().to_string();

            Command::new("git")
                .args(["pull", "--quiet",&top_level])
                .output()
                .expect("failed to fetch private repo. Have you cloned the private repo containing the secret keys?");

            // println!("Private repo file: {}", path_str);

            match load_regex_from_file(&path) {
                Ok(regexes) => {
                    println!("Loaded {} regexes", regexes.len());
                    patterns.extend(regexes);
                }
                Err(err) => {
                    println!("Error reading {}: {}", path_str, err);
                }
            }
        }
    }
    patterns
}

// add provider lists
pub fn add_config(path: &str, private: bool) {
    let key = if private {
        "git-find.private-file"
    } else {
        "git-find.regex-file"
    };

    // get all existing values
    let output = Command::new("git")
        .args(["config", "--global", "--get-all", key])
        .output()
        .expect("failed to run git config");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let exists = stdout.lines().any(|line| line.trim() == path);

    if exists {
        println!("key already exists");
    } else {
        println!("adding key path to git config");
        Command::new("git")
            .args(["config", "--global", "--add", key, path])
            .output()
            .expect("failed to add a key");
    }
}
