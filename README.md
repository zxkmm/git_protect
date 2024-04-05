# git_push_protect
a tool to protect your git push from pushing into repos which isn't belong to you but you have write access.

# screenshots
Pushing into not your repo:  

![screenshots_push__forbid](https://github.com/zxkmm/git_push_protect/blob/main/tools/img.png?raw=true)   


Pushing into your own repo:    

![screenshots_push_allowed](https://github.com/zxkmm/git_push_protect/blob/main/tools/img_1.png?raw=true)


# usage
1. make sure you have rust and cargo installed (they should come together if you use package manager to install)
2.
```
cd ~
mkdir apps
mkdir apps/git_push_protect
cd ~ 
git clone https://github.com/zxkmm/git_push_protect.git
cd git_push_protect
cargo build --release
cp ./target/release/git_push_protect ~/apps/git_push_protect
sudo chmod +x ~/apps/git_push_protect/git_push_protect
```
3. set apps into PATH
```
echo 'export PATH=$PATH:~/apps/git_push_protect' >> ~/.bashrc
echo 'export PATH=$PATH:~/apps/git_push_protect' >> ~/.zshrc
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
alias ppush='git_push_protect'
alias gpush='git_push_protect'
```
after that you can run ppush or gpush to use it.
# why rust
I don't know how to write rust but i wanted to learn it, and this is my first rust program

# warning
- not tested on windows but should be work.
- If you are github or any other git host server user:
    - Your user name of the host account must match the username your local git config. Because there's no way to detetct your destination host username (GitHub have an account username which isn't same with the real git username)
    - Don't cinsider this tool as for "security" or "safety" usage. This is only for conscious check, it's not for pretecting from intentional attack.
