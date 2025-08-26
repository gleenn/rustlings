.PHONY: test
test:
	cargo test --all-features

.PHONY: run
run:
	# rustlings watch
	cargo run

.PHONY: watch
watch:
	rustlings watch
