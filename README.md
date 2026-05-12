# debug-opt-broke-app

debug and optimize broken app

## Использовавшиеся команды

cargo +nightly miri run --bin demo
cargo +nightly miri test
valgrind --leak-check=full target/debug/demo
valgrind --leak-check=full cargo test --tests
RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --bin demo

export RUSTFLAGS=-Zsanitizer=thread RUSTDOCFLAGS=-Zsanitizer=thread
cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu

### Баги [файл](artifacts/FOUND_BUGS.md)

### Отчеты до оптимитации [файл](artifacts/before_optimize)

### Отчеты после оптимизации [файл](artifacts/after_optimize)

### Эталон https://github.com/wayfar9r/reference-app.git
