alias c := check
alias t := test
alias d := doc
alias f := format
alias fmt := format

default:
    just --list

ci: check test doc format

check:
    cargo clippy -- -Dwarnings

test:
    cargo test

doc:
    cargo doc --all-features --no-deps --document-private-items --keep-going

format:
    cargo fmt --check
