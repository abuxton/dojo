# https://github.com/krisnova/Makefile/blob/main/Makefile

-include .env

all: help

build: ## Build target does build things.
	@echo "Building..."

example: ## Example target does example things.
	@echo "Example..."

sample: ## Sample target does sample things.
	@echo "Sample..."

.PHONY: help
help:  ## Show help messages for make targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(MAKEFILE_LIST)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}'
