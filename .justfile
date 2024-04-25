alias d := dev
alias t := test

default:
  just --list

dev:
    clear
    typos
    cargo fmt --all -- --check
    cargo machete
    cargo deny check licenses
    cargo deny check bans --hide-inclusion-graph
    cargo deny check advisories
    cargo deny check sources
    cargo clippy --workspace --all-targets --all-features
    cargo build --workspace  --keep-going --all-features --timings

test:
    cargo test --workspace --all-targets --all-features
    cargo miri test --workspace --all-features
