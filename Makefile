.ONESHELL:
    
build-release:
	cargo build --release
    
build-wasm:
	cd ./wasm-binding && wasm-pack build
    
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