build:
    nix build

cbuild:
    cargo build
run command:
    nix run -- {{ command }}

crun command:
    cargo run -- {{ command }}
