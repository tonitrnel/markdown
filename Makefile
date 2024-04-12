.ONESHELL:

test-blocks:
	cargo test --package markdown --lib blocks

test-inlines:
	cargo test --package markdown --lib inlines 