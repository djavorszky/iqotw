# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources
check:
	cargo clippy --locked -- -D warnings

# Runs unit tests
test:
	cargo test --locked

# Runs the benchmarks
bench filter='':
  cargo bench --locked -- {{filter}}
