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

## Install
To install nix globally on any nixos machine that uses flakes is as easy as:
```nix
inputs.kix.url = "github:secretval/kix";
```
and then either:
```nix
environment.systemPackages = [inputs.kix.packages.${system}.default];
```
or
```nix
home.packages = [inputs.kix.packages.${system}.default];
```
You will have to create a variable `system` for that to work or you can just replace the `${system}` with your system architecture.
