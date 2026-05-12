# debug-opt-broke-app

debug and optimize broken app

## Использовавшиеся команды

cargo +nightly miri run --bin demo
cargo +nightly miri test
valgrind --leak-check=full target/debug/demo

### Баги [файл](artifacts/FOUND_BUGS.md)

### Отчеты до оптимитации [файл](artifacts/before_optimize)

### Отчеты после оптимизации [файл](artifacts/after_optimize)

### Эталон https://github.com/wayfar9r/reference-app.git
