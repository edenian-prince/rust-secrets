use colored::Colorize;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

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
pub fn read_git_regex_files() -> PathBuf {
    let output = Command::new("git")
        .args(["config", "--global", "--get", "git-find.regex-file"])
        .output()
        .expect("failed to find provider");

    PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
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

pub fn pre_commit_hook_scan(custom_patterns: Option<Regex>) {
    // Read in the global regex - this will fail if it's is_empty
    let global_secret_path = read_git_regex_files();
    let global_secrets = load_regex_from_file(&global_secret_path);

    let mut patterns = Vec::new();
    patterns.extend(global_secrets.unwrap());

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
        eprintln!("{}", "Secret scan failed. Commit aborted.".red().bold());
    } else {
        println!("{}", "No secrets found.".green().bold());
        std::process::exit(0);
    }
}

pub fn install_hooks(repo_url: &str, secrets_path: &str) {
    write_git_regex_file(repo_url, secrets_path);

    let mut git_template = env::home_dir().expect("Could not find home directory");
    git_template.push(".git-template/hooks/pre-commit");

    if git_template.is_file() {
        println!("pre-commit hook exists; adding hook to it");

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
    } else {
        println!("pre-commit hook file not found... creating file")
    }
}
