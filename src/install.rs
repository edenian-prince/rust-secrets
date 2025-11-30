use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn git_config_global_get(key: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--global", "--get", key])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let val = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if val.is_empty() { None } else { Some(val) }
}

pub fn determine_global_hooks_dir() -> PathBuf {
    // See if user already has a core.hooksPath
    if let Some(existing) = git_config_global_get("core.hooksPath") {
        let existing_path = PathBuf::from(existing.trim());
        println!("Detected existing core.hooksPath at {:?}", existing_path);
        println!("Will install hooks into this directory (not overriding).");
        return existing_path;
    }

    // Otherwise set our own hooks directory
    let home = env::home_dir().unwrap();
    let hooks_dir = home.join(".git-hooks");

    fs::create_dir_all(&hooks_dir).expect("Failed to create hooks directory");

    println!(
        "No global hooksPath found. Setting core.hooksPath to {:?}",
        hooks_dir
    );

    Command::new("git")
        .args([
            "config",
            "--global",
            "core.hooksPath",
            hooks_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to set global core.hooksPath");

    hooks_dir
}

pub fn global_hook_script() -> String {
    r#"#!/bin/sh
# Global pre-commit hook wrapper

# Run global logic
echo "Running global git-find hook"
git find hook
STATUS=$?

# If git-find failed, abort the commit immediately
if [ $STATUS -ne 0 ]; then
    exit $STATUS
fi

# Detect the repo root
GIT_DIR=$(git rev-parse --git-dir) || exit 0

# Look for a local hook
LOCAL_HOOK="$GIT_DIR/hooks/pre-commit"

if [ -f "$LOCAL_HOOK" ] && [ -x "$LOCAL_HOOK" ]; then
    echo "Running local pre-commit hook"
    "$LOCAL_HOOK" "$@"
    LOCAL_STATUS=$?

    # Only propagate local failure if it failed
    if [ $LOCAL_STATUS -ne 0 ]; then
        exit $LOCAL_STATUS
    fi
fi

exit 0
"#
    .to_string()
}

pub fn new_sh_script() -> String {
    r#"#!/bin/sh
# Global pre-commit hook (git-find)

TEMPLATE="$HOME/.git-hooks/git-find-global-hook.sh"

if [ -f "$TEMPLATE" ] && [ -x "$TEMPLATE" ]; then
    "$TEMPLATE" "$@"
    exit $?
fi

exit 0
"#
    .to_string()
}

pub fn existing_sh_script() -> String {
    r#"
# Added by git-find
TEMPLATE="$HOME/.git-hooks/git-find-global-hook.sh"
if [ -f "$TEMPLATE" ] && [ -x "$TEMPLATE" ]; then
    "$TEMPLATE" "$@"
    exit $?
fi
"#
    .to_string()
}

fn handle_global_pre_commit(path: &PathBuf) {
    if !path.exists() {
        println!("pre-commit does not exist, creating it");
        fs::write(path, new_sh_script()).expect("failed to write new pre-commit");
    } else {
        println!("pre-commit exists, checking for git-find include...");

        let contents = fs::read_to_string(path).expect("failed to read pre-commit");

        if !contents.contains("git-find-global-hook.sh") {
            println!("pre-commit missing git-find include, appending...");

            let mut file = OpenOptions::new()
                .append(true)
                .open(path)
                .expect("could not open pre-commit");

            file.write_all(existing_sh_script().as_bytes())
                .expect("failed to append script");
        } else {
            println!("git-find already included, skipping");
        }
    }

    Command::new("chmod")
        .args(["+x", path.to_str().unwrap()])
        .status()
        .expect("failed to chmod hook");
}

fn ensure_template_exists(path: &PathBuf) {
    if !path.exists() {
        println!("creating git-find-global-hook.sh");
        fs::write(path, global_hook_script()).expect("failed to write template script");
    }

    Command::new("chmod")
        .args(["+x", path.to_str().unwrap()])
        .status()
        .expect("failed to chmod template");
}

pub fn install_hooks() {
    let hooks_dir = determine_global_hooks_dir();

    let template_path = hooks_dir.join("git-find-global-hook.sh");
    ensure_template_exists(&template_path);

    let pre_commit = hooks_dir.join("pre-commit");
    handle_global_pre_commit(&pre_commit);

    println!("git-find hooks installed safely");
}
