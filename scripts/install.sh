#!/usr/bin/env bash
# install — build the repo tooling. No network beyond crates.io.
#
# This builds the map checker and the route driver in this repository. It does not
# build the inference engine: that ships as a hash-pinned binary, and the reason is
# stated in DETERMINISM.md rather than hidden.
#
# Exit 0 built, 2 environment.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

say "niodoo-adaptive-agency — install"

step "Toolchain"
require_cmd cargo "install rust: https://rustup.rs" || exit "$EX_ENV"

step "Build"
say "  cargo build --release --locked"
if cargo build --release --locked --manifest-path "$REPO_ROOT/Cargo.toml"; then
  ok "built"
else
  fail "$EX_ENV" "build failed"
fi

step "Tests"
if cargo test --locked --manifest-path "$REPO_ROOT/Cargo.toml" 2>&1 | tail -5; then
  ok "tests passed"
else
  warn "tests reported a failure — see output above"
fi

step "Next"
note "./run doctor        check this machine"
note "./run fetch         download the artifacts"
note "./run verify --check   assert the record, in seconds, without a GPU"
