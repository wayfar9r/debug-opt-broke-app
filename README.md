# debug-opt-broke-app

debug and optimize broken app

## Использовавшиеся команды

cargo +nightly miri run --bin demo

cargo +nightly miri test

valgrind --leak-check=full target/debug/demo [file](artifacts/valgrind_report.txt)

valgrind --leak-check=full cargo test --tests [file](artifacts/valgrind_test.report.txt)

RUSTFLAGS="-Zsanitizer=address" cargo +nightly run --bin demo [file](artifacts/sanitizer_address_report.txt)

export RUSTFLAGS=-Zsanitizer=thread RUSTDOCFLAGS=-Zsanitizer=thread
cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu [file](artifacts/sanitizer_thread_report.txt)

### Баги [файл](artifacts/FOUND_BUGS.md)

### Отчеты до оптимитации [файл](artifacts/before_optimize)

### Отчеты после оптимизации [файл](artifacts/after_optimize)

### Эталон https://github.com/wayfar9r/reference-app.git
