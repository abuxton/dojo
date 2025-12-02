# https://github.com/krisnova/Makefile/blob/main/Makefile

-include .env
REPO_TOP=$(shell git rev-parse --show-toplevel)
CORE=${REPO_TOP}/common/mk/core.mk
BIN_DIR=${REPO_TOP}/common/bin


# --- configurable variables ---
CARGO ?= cargo
CARGO_FLAGS ?=
CARGO_BUILD_FLAGS ?= --workspace $(CARGO_FLAGS)
CARGO_TEST_FLAGS ?= $(CARGO_FLAGS)
CARGO_FMT_FLAGS ?= --all
CARGO_CLIPPY_FLAGS ?= --all-targets --all-features -- -D warnings
RUSTDOC_FLAGS ?=
RUSTFLAGS ?=

# --- basic ops ---
.PHONY: rust-build rust-build-debug rust-clean rust-check

rust-build: ## Build release (alias)
	@echo "cargo build --release"
	$(CARGO) build --release $(CARGO_FLAGS)

rust-build-debug: ## Build debug
	@echo "cargo build"
	$(CARGO) build $(CARGO_BUILD_FLAGS)

rust-clean: ## Clean artifacts
	@echo "cargo clean"
	$(CARGO) clean

rust-check: ## Fast check (compilation check)
	@echo "cargo check"
	$(CARGO) check $(CARGO_FLAGS)

# --- test & bench ---
.PHONY: rust-test rust-test-release rust-bench
rust-test: ## Run unit tests (debug)
	@echo "cargo test"
	$(CARGO) test $(CARGO_TEST_FLAGS)

rust-test-release: ## Run tests with --release
	@echo "cargo test --release"
	$(CARGO) test --release $(CARGO_TEST_FLAGS)

rust-bench: ## Run benches (requires --bench targets defined)
	@echo "cargo bench"
	$(CARGO) bench $(CARGO_FLAGS)

# --- linting & formatting ---
.PHONY: rust-fmt rust-fmt-check rust-clippy
rust-fmt: ## Format all Rust files
	@echo "cargo fmt $(CARGO_FMT_FLAGS)"
	$(CARGO) fmt $(CARGO_FMT_FLAGS)

rust-fmt-check: ## Check formatting (use in CI)
	@echo "cargo fmt -- --check"
	$(CARGO) fmt -- $(CARGO_FMT_FLAGS) --check

rust-clippy: ## Run clippy and deny warnings
	@echo "cargo clippy $(CARGO_CLIPPY_FLAGS)"
	$(CARGO) clippy $(CARGO_CLIPPY_FLAGS)

# --- docs ---
.PHONY: rust-doc rust-doc-open
rust-doc: ## Build docs for workspace
	@echo "cargo doc --no-deps"
	$(CARGO) doc --no-deps $(RUSTDOC_FLAGS)

rust-doc-open: rust-doc ## Build and open docs (platform open)
	@echo "cargo doc --no-deps --open"
	$(CARGO) doc --no-deps --open $(RUSTDOC_FLAGS)

# --- run / install / publish helpers ---
.PHONY: rust-run rust-run-args rust-install
rust-run: ## Run binary in current crate (pass ARGS via make ARGS="-- foo")
	@echo "cargo run -- $(ARGS)"
	$(CARGO) run -- $(ARGS)

rust-run-args: ## Run with release (pass ARGS)
	@echo "cargo run --release -- $(ARGS)"
	$(CARGO) run --release -- $(ARGS)

rust-install: ## Install the crate (cargo install --path .)
	@echo "cargo install --path ."
	$(CARGO) install --path . $(CARGO_FLAGS)

# --- utilities & CI helpers ---
.PHONY: rust-ci
rust-ci: rust-fmt-check rust-clippy rust-test ## Run common CI checks

# print help for rust targets in this file
.PHONY: help-rust
help-rust:  ## Show help messages for make targets in ${BIN_DIR}/rust.mk
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(BIN_DIR)/rust.mk) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}'
