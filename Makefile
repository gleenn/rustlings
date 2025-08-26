.PHONY: run
run:
	# rustlings watch
	cargo run

.PHONY: test
test:
	cargo test --all-features

.PHONY: watch
watch:
	rustlings watch
