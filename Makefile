MUSL_BUILDER=docker run --rm -it -v "$(shell pwd)":/home/rust/src -v cargo-git:/home/rust/.cargo/git -v cargo-registry:/home/rust/.cargo/registry alichen/rust-musl-builder:0.0.1

install:
	@$(MUSL_BUILDER) sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry

build:
	@$(MUSL_BUILDER) cargo build --release

upload:
	@scp -P 22022 -C target/x86_64-unknown-linux-musl/release/aid aidapi@10.19.14.155:/home/aidapi/aid

.PHONY: install build upload
