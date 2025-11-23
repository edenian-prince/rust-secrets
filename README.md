# rust-secrets

Pre-commit hooks that prevent credential leaks, written in rust. A replacement of AWS git-secrets that also has automated provider refreshing. Like `git-secrets`, it adds a regex file to your git config and uses it to scan for secrets when you `git commit`. With `git-find`, it can automatically pull changes to that file before scanning for secrets, ensuring that you have the most up to date regex secret file. 

This is really useful for:

- teams that want to share a regex secret file (containing common server names, tokens, etc)
- teams that update their regex file and need to automatically update their teammates pre-commit hooks
- newbies that may forget to manually update their regex files

https://github.com/user-attachments/assets/94138c13-0102-42cf-a0c9-0be3481bd2c5

- git-find can automatically pull 'shared' regex provider files as shown in the video above. If an update is made to the centralized regex file, the new regex will be scanned against when your run git commit. This is great for teams that update what secrets they want to scan against and need to ensure that ALL team members have the latest regex file

https://github.com/user-attachments/assets/4c84368b-105c-4711-aeaf-f94902aecfaf

- has cleaner git history scanning capabilities (and will get better in future releases)

https://private-user-images.githubusercontent.com/113125657/517775645-5c318626-e415-46d5-9125-16efcc73b7f9.mp4?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NjM4NTgxNzYsIm5iZiI6MTc2Mzg1Nzg3NiwicGF0aCI6Ii8xMTMxMjU2NTcvNTE3Nzc1NjQ1LTVjMzE4NjI2LWU0MTUtNDZkNS05MTI1LTE2ZWZjYzczYjdmOS5tcDQ_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjUxMTIzJTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI1MTEyM1QwMDMxMTZaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT0zODcyM2Q1Y2FhNDRhMTUwMTFhNWZlMzRlZWNjYzY4ODQxMjIyZTJkN2RlZTY0NmEyNTZhNGE5NTZiZGYwNzQ1JlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.vwPv8uecDzH4Znp-z2LxuJdjNDwJCsVW-_KkGTriX6k

- automatically sets up global hooks that work on existing repos. AWS git-secrets was a real pain for this. when you install it you need to configure git to run it on existing repos. a pain for newbie git users

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

## Comparisons to GitLeaks and AWS Git Secrets

### GitLeaks

- Installing gitleaks gives you the binary/tool to scan for secrets, but it won’t “opt you in” with Git hooks.
- You need to explicitly configure the hook you want (pre-commit, protect, etc.) in each repository (or via a shared git template).
- This means a user will need to pip install pre-commit hook, set up a git template for global hooks, and then set up core.hooksPath to apply the hooks to existing repos

### AWS Git-Secrets

- Simple install
- Does not set up global hooks that apply to ALL repos (existing repos included, see video above)
- Does not have auto-config capabilities

**If you want to enforce hooks globally across many repos (e.g. for a team), you’ll need to set up a shared Git template directory or use a hook manager. This all handled by `git-find` when you run `git-find install`**

`git-find` simplifies installing global hooks and automating config pulls. It requires ZERO effort or knowledge from newbies that need pre-commit hooks for security scanning.



