# kix

## What is kix
Kix is a cli interface for [kickstart.nix](https://github.com/ALT-F4-LLC/kickstart.nix) written in rust (yeah rust)

## Install
Dependencies:
    - nix
    - nix-command
    - nix flakes
    - git
    - fzf

1. Install all the dependencies.
2. Clone this git repo.
3. In the repo you just cloned run:
```sh
nix profile install
```
4. Now you have kix installed.
5. Look here [How to use](#Quickstart)
6. Look here [How to update](#Updating)

## Quickstart
```sh
kix help
```
## Updating
1. Clone the repo again or do a git pull.
2. Then run this:
```sh
nix profile list
```
3. Copy the store path of the one with kix in the name.
4. Now run:
```sh
nix profile remove + {store path}
```
Replace {store path} with your store path.
5. Now you can run this again:
```sh
nix profile install
