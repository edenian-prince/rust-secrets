# rust-secrets

Pre-commit hooks that prevent credential leaks, written in rust. A replacement of AWS git-secrets that also has automated provider refreshing. Like `git-secrets`, it adds a regex file to your git config and uses it to scan for secrets when you `git commit`. With `git-find`, it can automatically pull changes to that file before scanning for secrets, ensuring that you have the most up to date regex secret file. 

This is really useful for:

- teams that want to share a regex secret file (containing common server names, tokens, etc)
- teams that update their regex file and need to automatically update their teammates pre-commit hooks
- newbies that may forget to manually update their regex files 

## Install 

### Linux/WSL

Run this in a bash terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/edenian-prince/rust-secrets/refs/heads/main/install.sh | bash
```

that will put the cli tool in your .bashrc

then restart your terminal or run 

```bash
source ~/.bashrc
```

### Windows

Run this in a PowerShell terminal. It will pull the `install.ps1` script from the repo and install `git-find.exe` to your `C:/USER/.local/bin` path

```PowerShell
powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/edenian-prince/rust-secrets/refs/heads/main/install.ps1 | iex"
```

## Setup

1. Once installed, run this (and then restart your shell if using PowerShell)

```bash
git find install
```

2. Add a secret provider. Can be either a .txt file on your local machine or a raw.github.txt file from github

```bash
git find add-provider --path /full/path/to/secret.txt
```

That's it!


### Optional:

#### Automatic git find add-provider

If you want an automated github regex file, you must first clone the repo and then run

```bash
git find add-provider --path /full/path/to/git/clone/secret.txt
```
This will prompt you and ask if you want the auto updates. Write Y and it will set it up for you. Whenever the pre-commit hook runs it will automatically pull from that repo so that your regex file is the most up to date.

#### git find scan

To scan the entire git history of a repo, run this within a git repo

```bash
git find scan
```

## Further Details

AWS Git Secrets is great, but for users new to git, it is not great. My team wanted to have one centralized regex file that all team members could scan against for
their pre-commit hooks. AWS git secrets can do that no problem, but it has no way of automatically pulling any _changes_ to the centralized file.

For example, 

- Person A: has the centralized regex file
- Person B: has the centralized regex file

- IT Security adds a new regex to the file..

- Person A: experienced in git, pulls the newest regex file into their AWS git secrets provider list
- Person B: **new to git, has no idea there is a new regex in the file or how to pull the changes...**

So, `git-find` makes this process super easy and requires ZERO effort or knowledge from newbies that need pre-commit hooks for security scanning.
It will automatically pull any changes from a centrailized provider/regex file and use those when scanning for secrets. See `src/utils/read_patterns()`



