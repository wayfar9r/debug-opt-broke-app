#!/usr/bin/env bash
set -euo pipefail

# Пример профилирования (Linux, perf). Настройте под свою систему.
cargo build --release
sudo perf record -g --call-graph dwarf ./target/release/demo || true
sudo chmod 777 perf.data
perf report
