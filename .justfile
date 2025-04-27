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
    taplo check
    taplo fmt --check
    cargo sort-derives --check
    just --fmt --check --unstable

[group("Code quality")]
check:
    cargo check --keep-going --workspace --all-features
    cargo clippy --workspace --all-targets --all-features -- -D warnings

[group("Code quality")]
dependencies:
    # cargo machete # TODO reenable
    cargo +nightly udeps --workspace --all-targets

[group("Code quality")]
compliance:
    # cargo pants --dev
    cargo deny check licenses
    cargo deny check bans --hide-inclusion-graph
    cargo deny check advisories
    cargo deny check sources

[group("Code quality")]
code-quality: lint check dependencies compliance
    cargo modules orphans --package bevy_mod_lockdown --all-features --deny

[group("Dev")]
dev: clear code-quality
    cargo build --workspace  --keep-going --all-features --timings

[group("Test")]
test: clear
    cargo nextest run --workspace --all-targets --all-features
    cargo miri test --workspace --all-features
