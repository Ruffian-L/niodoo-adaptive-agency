#!/usr/bin/env bash
# chat — interactive REPL with the durable store enabled.
#
#   ./run chat              start or continue the default session
#   ./run chat --fresh      start from an empty store
#   ./run chat --store PATH use a specific store file
#
# The store is a JSONL file on disk. It persists across restarts, and you can read
# it in a text editor at any time. Entries are written by the model when it emits a
# <remember> tag; nothing is written on your behalf.
#
# "Resume" here means two things, both readable on disk: the durable store, and a
# compact set of resume anchors. Raw transformer KV state is not persisted — it is
# large, fragile, and it would break the determinism the rest of this repo depends on.
#
# Exit 0 normal exit, 2 environment, 3 missing artifact.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

SESSION_DIR="$REPO_ROOT/runs/chat"
STORE="$SESSION_DIR/remember-store.jsonl"
ANCHORS="$SESSION_DIR/resume-anchors.json"
FRESH=0

while [ $# -gt 0 ]; do
  case "$1" in
    --fresh) FRESH=1; shift ;;
    --store) STORE="${2:?--store needs a path}"; shift 2 ;;
    -h|--help) sed -n '2,17p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) fail "$EX_ENV" "unknown option: $1" ;;
  esac
done

step "Artifacts"
for n in model tokenizer niodoo ghost-registry; do
  check_artifact "$n" >/dev/null || fail "$EX_MISSING" "$n unavailable — run ./run fetch"
done
ok "all required artifacts present and hash-verified"

mkdir -p "$SESSION_DIR" "$(dirname "$STORE")"

if [ "$FRESH" = "1" ]; then
  : > "$STORE"; rm -f "$ANCHORS"
  note "started a fresh store"
fi
[ -f "$STORE" ] || : > "$STORE"

entries_before="$(grep -c . "$STORE" 2>/dev/null || echo 0)"

step "Durable store"
say "  path:    $STORE"
say "  entries: $entries_before"
if [ "$entries_before" -gt 0 ]; then
  note "carried in from previous sessions:"
  sed 's/^/    /' "$STORE" | head -20
  [ "$entries_before" -gt 20 ] && note "    … and $((entries_before - 20)) more"
fi
[ -f "$ANCHORS" ] && note "resume anchors: $ANCHORS"

step "Session"
note "The model may emit <remember>key=value</remember> to write to the store above,"
note "and <lock>, <focus>, <explore>, <spike>, <reset> as control tags."
note "Type /quit or /exit to end. Entries per store line are capped at 120 characters."

ONE_SHOT_MODEL="$(artifact_path model)"
ONE_SHOT_TOKENIZER="$(artifact_path tokenizer)"
NIODOO="$(artifact_path niodoo)"
PRODUCT_ROOT="${ONE_SHOT_PRODUCT_ROOT:-$(dirname "$(dirname "$(dirname "$(dirname "$(dirname "$(artifact_path ghost-registry)")")")")")}"

# Only pass --compact-resume-state-load-file when the file actually exists; the
# engine treats a missing load path as an error rather than an empty session.
resume_args=()
if [ -s "$ANCHORS" ]; then
  resume_args+=(--compact-resume-state-load-file "$ANCHORS")
  note "resuming from $ANCHORS"
fi

NIODOO_REPO_ROOT="$PRODUCT_ROOT" \
"$NIODOO" \
  --model-path "$ONE_SHOT_MODEL" \
  --model-arch auto \
  --tokenizer-path "$ONE_SHOT_TOKENIZER" \
  --chat-template auto --system-prompt-mode free \
  --output-contract-mode off --context-length 8192 \
  --max-steps 512 --temperature 0.0 \
  --workspace-tools false \
  --chat-repl \
  --remember-store "$STORE" \
  --compact-resume-state-save-file "$ANCHORS" \
  "${resume_args[@]}"
rc=$?

entries_after="$(grep -c . "$STORE" 2>/dev/null || echo 0)"

step "Store on exit"
say "  entries: $entries_before → $entries_after"
if [ "$entries_after" -gt "$entries_before" ]; then
  ok "$((entries_after - entries_before)) new entr$([ $((entries_after-entries_before)) -eq 1 ] && echo y || echo ies) written this session"
  tail -n $((entries_after - entries_before)) "$STORE" | sed 's/^/    /'
else
  note "no new entries"
fi
note "$STORE persists. ./run chat continues from it; ./run chat --fresh starts over."
exit "$rc"
