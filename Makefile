.ONESHELL:

WASM_BINDING_DIR := wasm-binding
WASM_WEB_PKG_DIR := $(WASM_BINDING_DIR)/pkg-web
WASM_NODE_PKG_DIR := $(WASM_BINDING_DIR)/pkg-node
WASM_WRAPPER_DIR := $(WASM_BINDING_DIR)/npm
WASM_WRAPPER_TSC := $(WASM_WRAPPER_DIR)/node_modules/.bin/tsc
NPM_CACHE_DIR := /tmp/npm-cache-codex
WASM_VERSION := $(shell sed -n "s/^version = \"\(.*\)\"/\1/p" $(WASM_BINDING_DIR)/Cargo.toml | head -n 1)

test:
	echo "${WASM_VERSION}"

build-release:
	cargo build --release

$(WASM_WRAPPER_TSC): $(WASM_WRAPPER_DIR)/package.json $(WASM_WRAPPER_DIR)/package-lock.json
	cd $(WASM_WRAPPER_DIR)
	npm ci

wasm-build-web: $(WASM_WRAPPER_TSC)
	set -e
	wasm-pack build $(WASM_BINDING_DIR) --release --target bundler --out-dir pkg-web
	cp $(WASM_WRAPPER_DIR)/facade.ts $(WASM_WEB_PKG_DIR)/facade.ts
	cp $(WASM_WRAPPER_DIR)/index.ts $(WASM_WEB_PKG_DIR)/index.ts
	cp $(WASM_WRAPPER_DIR)/tsconfig.web.json $(WASM_WEB_PKG_DIR)/tsconfig.json
	$(WASM_WRAPPER_TSC) -p $(WASM_WEB_PKG_DIR)/tsconfig.json
	sed 's/__VERSION__/$(WASM_VERSION)/g' $(WASM_BINDING_DIR)/npm/package.web.json > $(WASM_WEB_PKG_DIR)/package.json

wasm-build-node: $(WASM_WRAPPER_TSC)
	set -e
	wasm-pack build $(WASM_BINDING_DIR) --release --target nodejs --out-dir pkg-node
	cp $(WASM_WRAPPER_DIR)/facade.ts $(WASM_NODE_PKG_DIR)/facade.ts
	cp $(WASM_WRAPPER_DIR)/index.ts $(WASM_NODE_PKG_DIR)/index.ts
	cp $(WASM_WRAPPER_DIR)/tsconfig.node.json $(WASM_NODE_PKG_DIR)/tsconfig.json
	$(WASM_WRAPPER_TSC) -p $(WASM_NODE_PKG_DIR)/tsconfig.json
	sed 's/__VERSION__/$(WASM_VERSION)/g' $(WASM_BINDING_DIR)/npm/package.node.json > $(WASM_NODE_PKG_DIR)/package.json

wasm-build-all: wasm-build-web wasm-build-node

wasm-bench: $(WASM_WRAPPER_TSC)
	set -e
	RUSTFLAGS='-C target-feature=+simd128' wasm-pack build $(WASM_BINDING_DIR) --release --target nodejs --out-dir pkg
	cp $(WASM_WRAPPER_DIR)/facade.ts $(WASM_BINDING_DIR)/pkg/facade.ts
	cp $(WASM_WRAPPER_DIR)/index.ts $(WASM_BINDING_DIR)/pkg/index.ts
	cp $(WASM_WRAPPER_DIR)/tsconfig.node.json $(WASM_BINDING_DIR)/pkg/tsconfig.json
	$(WASM_WRAPPER_TSC) -p $(WASM_BINDING_DIR)/pkg/tsconfig.json
	sed 's/__VERSION__/$(WASM_VERSION)/g' $(WASM_BINDING_DIR)/npm/package.node.json > $(WASM_BINDING_DIR)/pkg/package.json
	cd bench/compare/wasm
	npm ci
	npm run bench

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
	rm -f target/profiling/deps/timing_test-*
	cargo build --profile profiling --bench timing_test
	BIN=""; \
	for f in target/profiling/deps/timing_test-*; do \
		if [ -f "$$f" ] && [ -x "$$f" ]; then \
			BIN="$$f"; \
			break; \
		fi; \
	done; \
	test -n "$$BIN"; \
	samply record --rate 10000 "$$BIN"
	
