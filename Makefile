.ONESHELL:

WASM_BINDING_DIR := wasm-binding
WASM_WEB_PKG_DIR := $(WASM_BINDING_DIR)/pkg-web
WASM_NODE_PKG_DIR := $(WASM_BINDING_DIR)/pkg-node
NPM_CACHE_DIR := /tmp/npm-cache-codex
WASM_VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' $(WASM_BINDING_DIR)/Cargo.toml | head -n 1)

build-release:
	cargo build --release

wasm-build-web:
	wasm-pack build $(WASM_BINDING_DIR) --release --target bundler --out-dir pkg-web
	cd $(WASM_WEB_PKG_DIR)
	npm pkg set name=@ptdgrp/markdown-wasm version=$(WASM_VERSION)
	npm pkg set description="markdown wasm binding (browser/bundler)"
	npm pkg set publishConfig.access=public

wasm-build-node:
	wasm-pack build $(WASM_BINDING_DIR) --release --target nodejs --out-dir pkg-node
	cd $(WASM_NODE_PKG_DIR)
	npm pkg set name=@ptdgrp/markdown-wasm-node version=$(WASM_VERSION)
	npm pkg set description="markdown wasm binding (nodejs)"
	npm pkg set publishConfig.access=public

wasm-build-all: wasm-build-web wasm-build-node

wasm-pack-web: wasm-build-web
	mkdir -p $(NPM_CACHE_DIR)
	rm -f $(WASM_WEB_PKG_DIR)/*.tgz
	cd $(WASM_WEB_PKG_DIR)
	npm pack --cache $(NPM_CACHE_DIR)

wasm-pack-node: wasm-build-node
	mkdir -p $(NPM_CACHE_DIR)
	rm -f $(WASM_NODE_PKG_DIR)/*.tgz
	cd $(WASM_NODE_PKG_DIR)
	npm pack --cache $(NPM_CACHE_DIR)

wasm-pack-all: wasm-pack-web wasm-pack-node

wasm-publish-web:
	cd $(WASM_WEB_PKG_DIR)
	npm publish --access public

wasm-publish-node:
	cd $(WASM_NODE_PKG_DIR)
	npm publish --access public

wasm-publish-all: wasm-publish-web wasm-publish-node

# backward compatibility
build-wasm: wasm-build-web
publish-wasm: wasm-publish-web
    
test-blocks:
	cargo test --lib blocks
	cargo test --test escapes
	cargo test --test entity
	cargo test --test thematic_breaks
	cargo test --test headings
	cargo test --test code
	cargo test --test paragraphs
	cargo test --test blank_lines
	cargo test --test block_quotes
	cargo test --test lists
	cargo test --test html
	cargo test --test link_reference_definitions
	cargo test --test tabs

test-basics:
	cargo test --lib blocks
	cargo test --lib inlines

test-inlines:
	cargo test --lib inlines

test-passed: test-basics
	cargo test --test escapes
	cargo test --test entity
	cargo test --test thematic_breaks
	cargo test --test headings
	cargo test --test code
	cargo test --test paragraphs
	cargo test --test blank_lines
	cargo test --test block_quotes

samply:
	rm target/release/deps/timing_test-*
	cargo build --release --bench timing_test
	BIN=""
	for f in target/release/deps/timing_test-*; do
		if [ -f "$$f" ] && [ -x "$$f" ]; then
			BIN="$$f"
			break
		fi
	done
	test -n "$$BIN"
	samply record --rate 10000 "$$BIN"
	
