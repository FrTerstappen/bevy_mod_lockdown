[private]
@default:
    just --list

[group("Helper")]
[private]
clear:
    clear

[group("Code quality")]
lint:
    typos
    cargo fmt --all -- --check
    just --fmt --check --unstable
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo machete
    cargo udeps --workspace

[group("Code quality")]
check:
    cargo check --keep-going --workspace --all-features
    cargo pants
    cargo deny check licenses
    cargo deny check bans --hide-inclusion-graph
    cargo deny check advisories
    cargo deny check sources
    cargo outdated --depth 6

[group("Dev")]
dev: clear code-quality
    cargo build --workspace  --keep-going --all-features --timings

[group("Test")]
test: clear
    cargo nextest run --workspace --all-targets --all-features
    cargo miri test --workspace --all-features
