#!/usr/bin/env bash
# sweep — reshuffle the durable store and measure how often the rule still survives.
#
# The finding this reproduces: whether a stored rule is recovered depends on the
# ORDER of the entries around it, not only on how many there are. The same store,
# reshuffled, passes in some arrangements and fails in others.
#
#   ./run sweep                        shuffle the default store 6 times
#   ./run sweep --shuffles 12          more draws
#   ./run sweep --store PATH           sweep a store you made with teach or chat
#
# The rule entries are held at fixed positions in every arrangement; only the other
# entries move. Each run is a single draw: the pipeline is deterministic, so one run
# per arrangement is the complete answer for that arrangement.
#
# Exit 0 completed, 2 environment, 3 missing artifact.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

SHUFFLES=6
ENTRIES=""      # trim the store to N entries; empty means use all of it
STORE=""
DEST="$REPO_ROOT/flag/session.txt"
EXPECTED="$(mf expected gold_answer)"

while [ $# -gt 0 ]; do
  case "$1" in
    --shuffles) SHUFFLES="${2:?--shuffles needs a number}"; shift 2 ;;
    --entries)  ENTRIES="${2:?--entries needs a number}"; shift 2 ;;
    --store)    STORE="${2:?--store needs a path}"; shift 2 ;;
    --destination) DEST="${2:?}"; shift 2 ;;
    --expected) EXPECTED="${2:?}"; shift 2 ;;
    -h|--help) sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) fail "$EX_ENV" "unknown option: $1" ;;
  esac
done

say "niodoo-adaptive-agency — order sweep"

step "Artifacts"
for n in model tokenizer niodoo ghost-registry; do
  check_artifact "$n" >/dev/null || fail "$EX_MISSING" "$n unavailable — run ./run fetch"
done
ok "all required artifacts present and hash-verified"

# ---------------------------------------------------------------------------
# Choose the store to sweep.
# ---------------------------------------------------------------------------
step "Store"
# The store used in the original measurement is personal and is not in this
# repository. What ships is a synthetic set of matched length. The order effect does
# not depend on content — the same set is held fixed and only its permutation
# changes — so a stranger can reproduce that effect here. The separate content
# result (K=32) is NOT reproducible from this repository and is recorded as a named
# limitation in SWEEP.md rather than approximated with synthetic data.
SYNTH="$REPO_ROOT/reference/sweep-store-synthetic.jsonl"
if [ -z "$STORE" ]; then
  STORE="$SYNTH"
  note "using the bundled synthetic store ($(grep -c . "$SYNTH") entries)"
  note "for a result about your own store: ./run sweep --store PATH"
else
  note "using supplied store: $STORE"
fi
[ -s "$STORE" ] || fail "$EX_MISSING" "store not found or empty: $STORE"

RULE_STORE="$REPO_ROOT/flag/reearn-20260809/self-saved-store.jsonl"
[ -f "$RULE_STORE" ] || fail "$EX_MISSING" "rule store not found at $RULE_STORE"

WORK="$REPO_ROOT/runs/sweep-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$WORK/stores"
cp "$STORE" "$WORK/filler.jsonl"

if [ -n "$ENTRIES" ]; then
  head -n "$ENTRIES" "$WORK/filler.jsonl" > "$WORK/filler.trimmed" && mv "$WORK/filler.trimmed" "$WORK/filler.jsonl"
  note "trimmed to $ENTRIES entries"
fi
n_filler="$(grep -c . "$WORK/filler.jsonl")"
say "  rule entries: $(grep -c . "$RULE_STORE") (held at fixed positions)"
say "  other entries: $n_filler (reshuffled each run)"
say "  arrangements to test: $SHUFFLES"

# ---------------------------------------------------------------------------
step "Building arrangements"
python3 - "$RULE_STORE" "$WORK/filler.jsonl" "$WORK/stores" "$SHUFFLES" <<'PY'
import random, sys, pathlib
rule = [l for l in open(sys.argv[1]).read().splitlines() if l.strip()]
filler = [l for l in open(sys.argv[2]).read().splitlines() if l.strip()]
out = pathlib.Path(sys.argv[3]); n = int(sys.argv[4])
for i in range(1, n + 1):
    order = list(filler)
    random.Random(i * 7919).shuffle(order)      # deterministic per index, reproducible
    (out / f"arrangement_{i:02d}.jsonl").write_text("\n".join(rule + order) + "\n")
print(f"  wrote {n} arrangements")
PY

# ---------------------------------------------------------------------------
step "Running"
CSV="$WORK/results.csv"
echo "arrangement,status,answer_found,decode_tokens" > "$CSV"
passes=0; total=0

for s in "$WORK"/stores/arrangement_*.jsonl; do
  i="$(basename "$s" .jsonl | sed 's/arrangement_//')"
  rd="$WORK/run_$i"; mkdir -p "$rd"
  total=$((total+1))
  printf '  arrangement %s ... ' "$i"

  # _transfer_once.sh takes the store as its first argument and reads the
  # destination from ONE_SHOT_DESTINATION. It does not go through the full route,
  # so the route-level variables do not apply here.
  ONE_SHOT_DESTINATION="$DEST" \
    "$REPO_ROOT/scripts/_transfer_once.sh" "$s" "$rd" >"$rd/driver.log" 2>&1
  rc=$?

  tx="$rd/complete.stdout.txt"
  # A crash is not a FAIL. FAIL means the run completed and produced a wrong answer;
  # ERROR means it never got that far, and averaging the two would be meaningless.
  if [ "$rc" -ne 0 ] && [ ! -s "$tx" ]; then
    found=no; status=ERROR
  elif [ -f "$tx" ] && [ "$(grep -coF -- "$EXPECTED" "$tx")" -ge 1 ]; then
    found=yes; status=PASS; passes=$((passes+1))
  else
    found=no; status=FAIL
  fi
  tok="$(python3 -c "import json;print(json.load(open('$rd/telemetry.run_timing.json'))['decode_tokens'])" 2>/dev/null || echo "")"
  printf '%s  (%s tokens)\n' "$status" "${tok:-?}"
  echo "$i,$status,$found,${tok:-}" >> "$CSV"
done

# ---------------------------------------------------------------------------
step "Result"
errors="$(grep -c ',ERROR,' "$CSV" || true)"
say "  $passes of $total arrangements recovered the rule"
[ "${errors:-0}" -gt 0 ] && warn "$errors arrangement(s) did not complete and are excluded from the rate"
completed=$(( total - ${errors:-0} ))
if [ "$completed" -gt 0 ]; then
  pct=$(( passes * 100 / completed ))
  say "  pass rate: ${pct}% of $completed completed arrangements"
fi
say ""
note "Same entries, same count, different order. A single passing arrangement is one draw."
note "results: $CSV"
exit "$EX_OK"
