#!/usr/bin/env bash
set -euo pipefail


cargo build --release
sudo perf record -g --call-graph dwarf ./target/release/baseline || true
sudo chmod 777 perf.data
perf report
