SHELL := /bin/bash

.PHONY: all check test

all: check test

check:
	cargo build --no-default-features && \
	cargo clippy --no-default-features -- -D warnings && \
	cargo build --all-features && \
	cargo clippy --all-features -- -D warnings && \
	cargo build --examples && \
	cargo fmt --all -- --check

test:
	cargo test --no-default-features && \
	cargo test --all-features
