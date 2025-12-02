# https://github.com/krisnova/Makefile/blob/main/Makefile

-include .env
REPO_TOP=$(shell git rev-parse --show-toplevel)
CORE=${REPO_TOP}/common/mk/core.mk
BIN_DIR=${REPO_TOP}/common/bin

rust-build: ## Build Rust project
	@echo "Building Rust project..."
	cargo build --release


.PHONY: help-rust
help-rust:  ## Show help messages for make targets in ${BIN_DIR}/rust.mk
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(BIN_DIR)/rust.mk) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}'
