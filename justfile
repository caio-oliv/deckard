default:
    @just --list

test-feature-matrix:
    cargo test --all --no-default-features

test-cov:
    cargo llvm-cov clean
    cargo llvm-cov test --workspace --all-features --html

test-cov-open:
    python -m http.server 9010 -d target/llvm-cov/html
