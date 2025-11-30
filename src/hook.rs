use colored::Colorize;
use regex::Regex;

use crate::utils;

pub fn pre_commit_hook_scan(custom_patterns: Option<Regex>) {
    // Read in the global regex - this will fail if it's is_empty
    let global_secrets = utils::read_patterns();

    let mut patterns = Vec::new();
    patterns.extend(global_secrets);

    if let Some(re) = custom_patterns {
        patterns.push(re);
    }

    // now we need to flip through the staged git files and search for regex matches
    let staged_files = utils::get_staged_files();

    let mut secrets_found = false;

    for file in &staged_files {
        if let Some(content) = utils::get_staged_content(file) {
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
