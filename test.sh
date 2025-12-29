#!/usr/bin/env bash
set -euo pipefail

# Runs the Koto algebraeon test suite with tests enabled.
cargo run -- --tests tests/koto/algebraeon_tests.koto "$@"
