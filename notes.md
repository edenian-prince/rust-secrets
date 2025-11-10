in the hook, it will run `git find --path path/to/config/regex`

and it will say if this needs to be overwritten, write `git commit --no-verify`

so now we need a few things,
- in the install, have a prompt that asks where the git config file path is
- then it will clone that path and take the secrets file and put it into the .config/gitfind/regex.txt 
- it should have similar names to AWS git secrets 

