SHELL := /bin/bash

.PHONY: all check test

all: check test

check:
	cargo build --all-features && \
	cargo clippy --all-features -- -D warnings && \
	cargo fmt --all -- --check

test:
	cargo test --all-features
