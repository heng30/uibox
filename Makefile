#!/bin/bash

all:
	SLINT_STYLE=fluent cargo build --release

build:
	SLINT_STYLE=fluent cargo build --release

build-timings:
	SLINT_STYLE=fluent cargo build --release --timings
	cp -rf ./target/cargo-timings/cargo-timing.html ./profile

build-debug:
	SLINT_STYLE=fluent cargo build

run:
	SLINT_STYLE=fluent RUST_LOG=error,warn,info,debug cargo run

run-local:
	RUST_LOG=error,warn,info,debug ./target/debug/uibox

run-local-release:
	RUST_LOG=error,warn,info,debug./target/release/uibox

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental/*

clean:
	cargo clean

install:
	cp -rf ./target/release/uibox ~/bin/

slint-view:
	slint-viewer --style fluent --auto-reload -I uibox/ui ./uibox/ui/appwindow.slint
