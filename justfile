default: run

# Lint files and run tests with optional specific test cases
test *tests: lint
  # Runs the specified tests, or all tests if no specific test is provided
  cargo test {{tests}}

# Update submodules recursively
update:
  # Initializes and updates all git submodules
  git submodule update --init --recursive

# Bootstrap the environment by updating submodules and installing cross
bootstrap *aoc_token: update
  cargo install cargo-aoc

credentials aoc_token: bootstrap
  cargo aoc credentials {{aoc_token}}

# Run linting tools (cargo fmt and clippy) to ensure code style and correctness
lint:
  # Formats the code and checks for common mistakes with clippy
  cargo fmt && cargo clippy

# Fix lint issues automatically and allow dirty or staged files
lint_fix:
  # Formats the code and attempts to fix clippy errors automatically
  cargo fmt && cargo clippy --fix --allow-dirty --allow-staged

fetch *day:
  cargo aoc input {{ if day != "" { "-d" } else { "" } }} {{day}}

run *day: lint test
  cargo aoc {{ if day != "" { "-d" } else { "" } }} {{day}}