use std::path::Path;
use std::process::Command;

pub fn list_providers() {
    let gc = Command::new("git")
        .args(["config", "--global", "--get-regex", "git-find.*"])
        .output()
        .expect("listing providers failed.. do you have providers set up?");

    println!("{}", String::from_utf8_lossy(&gc.stdout));
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
