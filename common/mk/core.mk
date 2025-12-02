# https://github.com/krisnova/Makefile/blob/main/Makefile

-include .env
REPO_TOP=$(shell git rev-parse --show-toplevel)
CORE=${REPO_TOP}/common/mk/core.mk
BIN_DIR=${REPO_TOP}/common/bin


all: help help-git help-rust


-include ${REPO_TOP}/common/bin/git.mk
-include ${REPO_TOP}/common/bin/rust.mk


.PHONY: help
help:  ## Show help messages for make targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(firstword $(CORE)) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}'


