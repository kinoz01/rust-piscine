# only dirs directly under tests/, ignore target/
EXERCISES := $(patsubst %_test,%,$(notdir $(wildcard tests/*_test)))
# solved ones
SOLVED    := $(notdir $(wildcard solutions/*))
TESTABLE  := $(filter $(EXERCISES),$(SOLVED))

help:
	@echo "Usage:"
	@echo "  make <exercise>		- Run tests for the exercise"
	@echo "  make test-all			- Run tests for all exercises in solutions/"
	@echo "  make dir-<exercise>		- Create solutions/<exercise> dir with main.rs + lib.rs"
	@echo "  make run-<exercise>		- Run the program on solutions/<exercise> dir"
	@echo "  make ruspdate			- Install/upgrade rustup (no sudo); prints 'source \$\$HOME/.cargo/env'"
	@echo "  make cargo-edition YEAR=2021	- Bulk-change edition in all Cargo.toml files under solutions/ (in case you face edition issues)"

$(EXERCISES):
	@cargo test --manifest-path tests/$@_test/Cargo.toml

$(EXERCISES:%=dir-%):
	@set -e; \
	name=$(patsubst dir-%,%,$@); \
	dir=solutions/$$name; \
	test_main=tests/$$name"_test"/src/main.rs; \
	if [ -d $$dir ]; then \
		echo "$$dir already exists. Skipping."; \
	else \
		echo "Creating $$dir …"; \
		cargo new --lib $$dir; \
		# copy import + fn main, stop at the first #[cfg(test)] \
		if [ -f $$test_main ]; then \
			awk '/^[[:space:]]*#\[cfg\(test\)\]/{exit} {print}' \
				$$test_main > $$dir/src/main.rs; \
		else \
			echo "// main stub" > $$dir/src/main.rs; \
		fi; \
		: > $$dir/src/lib.rs; \
		echo "Created $$dir"; \
	fi

$(TESTABLE:%=run-%):
	@name=$(patsubst run-%,%,$@); \
	echo "Running $$name …"; \
	cargo run --manifest-path solutions/$$name/Cargo.toml

rust:
	@command -v rustc >/dev/null 2>&1 && { \
		echo "Rust is already installed (rustc found). Skipping installation."; \
	} || { \
		echo "Rust not found. Installing..."; \
		curl https://sh.rustup.rs -sSf | sh -s -- -y; \
		echo "Rust installed. To use it now, reopen shell or run:"; \
		echo "	source $$HOME/.cargo/env"; \
	}

test-all:
	@fails=""; passes=""; \
	list="$(TESTABLE)"; \
	if [ -z "$$list" ]; then \
		echo "No solved exercises in solutions/"; exit 0; \
	fi; \
	for ex in $$list; do \
		echo "==> testing $$ex"; \
		if cargo test --manifest-path tests/$${ex}_test/Cargo.toml; then \
			passes="$$passes $$ex"; \
		else \
			fails="$$fails $$ex"; \
		fi; \
	done; \
	echo ""; \
	echo "===== Summary ====="; \
	[ -n "$$passes" ] && echo "Passed:$$passes"; \
	[ -n "$$fails"  ] && echo "Failed:$$fails"; \
	[ -z "$$fails" ] || exit 1

# Install/upgrade rustup & Rust toolchain without sudo, then remind to source env
update-rust:
	@command -v rustup >/dev/null 2>&1 || curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
	@echo 'Run: source $$HOME/.cargo/env'

# Usage: make cargo-edition YEAR=<year>   (default YEAR=2024)
YEAR ?= 2024
cargo-edition:
	@find solutions -maxdepth 2 -name Cargo.toml -exec sed -i \
	  -e '/^cargo-features *= *\["edition2024"\]/d' \
	  -e 's/edition *= *"[0-9][0-9][0-9][0-9]"/edition = "$(YEAR)"/' {} +