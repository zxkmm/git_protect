# git_push_protect
a tool to protect your git push from pushing into repos which isn't belong to you but you have write access.

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
not tested on windows but should be work.
