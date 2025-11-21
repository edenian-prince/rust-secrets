# rust-secrets

Pre-commit hooks written in rust. A complete replacement of AWS git-secrets

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
powershell -ExecutionPolicy ByPass -c "irm https://github.com/edenian-prince/rust-secrets/blob/main/install.ps1 | iex"
```

## Details

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



