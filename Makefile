.PHONY: help
help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; \
		{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the workspace in debug mode
	cargo build --workspace --all-targets

.PHONY: build-release
build-release: ## Build the workspace in release mode
	cargo build --workspace --all-targets --release

.PHONY: check
check: check-format lint test ## Run all checks (format, lint, tests)

.PHONY: check-format
check-format: ## Check Rust code formatting
	cargo fmt --all -- --check

.PHONY: format
format: ## Format Rust code
	cargo fmt --all

.PHONY: lint
lint: ## Run clippy with denied warnings
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: lint-fix
lint-fix: ## Run clippy with auto-fix
	cargo clippy --workspace --all-targets --fix --allow-dirty \
		--allow-staged

.PHONY: test
test: ## Run tests
	cargo test --workspace --all-targets

.PHONY: test-doc
test-doc: ## Run doc tests
	cargo test --workspace --doc

.PHONY: doc
doc: ## Build documentation
	cargo doc --workspace --no-deps --document-private-items

.PHONY: doc-open
doc-open: ## Build and open documentation in browser
	cargo doc --workspace --no-deps --document-private-items --open

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: setup
setup: ## Setup development environment
	rustup component add rustfmt clippy
	@echo "Setup complete."

.PHONY: tree
tree: ## Show workspace layout (excluding target/)
	@find . -type d -not -path './target*' -not -path './.git*' \
		-not -path '*/node_modules*' | sort
