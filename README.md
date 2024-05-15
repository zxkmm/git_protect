# git_protect
A local tool to protect you performing git commands carefully (protect git push or other command)

# branches
You have several branches to choose from, they are having different features:

## Method 1: protect all git commands
- cover all git command with another command, e.g.: `pgit commit -m "message"` or `pgit push`
- it will check if you are in the right repo before executing any git command

### screenshots

Pushing into your own repo:

![screenshots_push_allowed1](https://github.com/zxkmm/git_push_protect/blob/cover_all_git_command/tools/img_2.png?raw=true)


Pushing into not your repo:

![screenshots_push_forbid1](https://github.com/zxkmm/git_push_protect/blob/cover_all_git_command/tools/img_3.png?raw=true)


### usage
1. make sure you have rust and cargo installed (they should come together if you use package manager to install)
2.
```
cd ~
mkdir apps
mkdir apps/git_push_protect
cd ~ 
git clone https://github.com/zxkmm/git_protect.git
cd git_protect
git checkout cover_all_git_command
cargo build --release
cp ./target/release/git_protect ~/apps/git_protect
sudo chmod +x ~/apps/git_protect/git_protect
```
3. set apps into PATH
```
echo 'export PATH=$PATH:~/apps/git_protect' >> ~/.bashrc
echo 'export PATH=$PATH:~/apps/git_protect' >> ~/.zshrc
source ~/.bashrc
source ~/.zshrc
# or whatever shell you use
```
4. next time, when you want push, use `git_protect` instead of `git push`.
5. you can also set alias:
```
vim ~/.zshrc
# or
nvim ~/.zshrc
# or
vi ~/.zshrc
# or
emacs ~/.zshrc
# add the following line
alias pgit='git_protect'
alias gitp='git_protect'
```
after that you can run pgit or gitp to use it, e.g.: `pgit push` or `pgit commit -m "message"`

## Method 2: protect only git push command
- only protect git push command, e.g.: `gpush` or `ppush`
- it will check if you are in the right repo before executing git push command

### screenshots
Pushing into not your repo:  

![screenshots_push__forbid](https://github.com/zxkmm/git_push_protect/blob/main/tools/img.png?raw=true)   


Pushing into your own repo:    

![screenshots_push_allowed](https://github.com/zxkmm/git_push_protect/blob/main/tools/img_1.png?raw=true)


### usage
1. make sure you have rust and cargo installed (they should come together if you use package manager to install)
2.
```
cd ~
mkdir apps
mkdir apps/git_push_protect
cd ~ 
git clone https://github.com/zxkmm/git_protect.git
cd git_protect
git checkout only_protect_git_push
cargo build --release
cp ./target/release/git_protect ~/apps/git_protect
sudo chmod +x ~/apps/git_protect/git_protect
```
3. set apps into PATH
```
echo 'export PATH=$PATH:~/apps/git_protect' >> ~/.bashrc
echo 'export PATH=$PATH:~/apps/git_protect' >> ~/.zshrc
source ~/.bashrc
source ~/.zshrc
# or whatever shell you use
```
4. next time, when you want push, use `git_push_protect` instead of `git push`.
5. you can also set alias:
```
vim ~/.zshrc
# or
nvim ~/.zshrc
# or
vi ~/.zshrc
# or
emacs ~/.zshrc
# add the following line
alias ppush='git_protect'
alias gpush='git_protect'
```
after that you can run ppush or gpush to use it.
# why rust
I don't know how to write rust but i wanted to learn it, and this is my first rust program

# warning
- not tested on windows but should be work.
- If you are GitHub or any other git host server user:
    - Your username of the host account must match the username your local git config. Because there's no way to detetct your destination host username (gh cli client is too slow responding to fetch it) (GitHub have an account username which isn't same thing with the real git username)
    - Don't consider this tool as for "security" or "safety" usage. This tool is only for conscious check, it's not for pretecting from intentional attack.
