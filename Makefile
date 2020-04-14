MUSL_BUILDER=docker run --rm -it -v "$(shell pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry alichen/rust-musl-builder:latest

install:
	@$(MUSL_BUILDER) sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry

build:
	@$(MUSL_BUILDER) cargo build --release

.PHONY: install build