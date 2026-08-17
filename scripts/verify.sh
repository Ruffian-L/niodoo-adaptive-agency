#!/usr/bin/env bash
# verify — the headline command. Two modes:
#
#   ./run verify --check   assert the recorded run satisfies every claim made about it.
#                          Seconds. No GPU, no model, no network. Start here.
#
#   ./run verify           re-execute the full route on this machine and diff the
#                          result against the recorded run. Minutes. Needs the GPU.
#
# What --check proves: the published record is internally consistent and says what
# the paper says it says. What the full run proves: this machine reproduces it.
#
# Replaying recorded stdin reproduces the computation, not the teaching. A reader who
# only replays these bytes learns that the bytes are real. `./run sweep` is the lane
# that produces an independent result on the reader's own hardware.
#
# Exit 0 pass, 1 mismatch, 2 environment, 3 missing artifact.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

MODE=full
while [ $# -gt 0 ]; do
  case "$1" in
    --check) MODE=check; shift ;;
    -h|--help) sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) fail "$EX_ENV" "unknown option: $1" ;;
  esac
done

REF="$(artifact_usable_path reference-run)"
GOLD="$(mf expected gold_answer)"
GOLD_LINE="$(mf expected gold_line)"
GOLD_N="$(mf expected gold_occurrences)"
WANT_STATUS="$(mf expected score_status)"
WANT_ORACLE="$(mf expected oracle_gate)"
WANT_STORE_SHA="$(python3 "$REPO_ROOT/scripts/manifest.py" contains reference-run \
                   | awk -F'\t' '$1=="teach/self-saved-store.jsonl"{print $2}')"
WANT_TX_SHA="$(python3 "$REPO_ROOT/scripts/manifest.py" contains reference-run \
                 | awk -F'\t' '$1=="transfer/r1/complete.stdout.txt"{print $2}')"
WANT_TX_BYTES="$(python3 "$REPO_ROOT/scripts/manifest.py" contains reference-run \
                   | awk -F'\t' '$1=="transfer/r1/complete.stdout.txt"{print $3}')"

pass_n=0; fail_n=0
assert_ok()   { ok "$1";  pass_n=$((pass_n+1)); }
assert_bad()  { bad "$1"; fail_n=$((fail_n+1)); [ -n "${2:-}" ] && note "$2"; [ -n "${3:-}" ] && note "$3"; }

if [ ! -d "$REF" ]; then
  bad "reference run not available as an extracted tree"
  note "resolved to: $REF"
  case "$REF" in
    *.tar.*) note "that is the archive; it has not been unpacked. Run: ./run fetch" ;;
    *)       note "run ./run fetch, or set ONE_SHOT_REFERENCE_RUN to the extracted directory" ;;
  esac
  exit "$EX_MISSING"
fi

say "niodoo-adaptive-agency — verify (mode: $MODE)"
say "reference: $REF"

# ---------------------------------------------------------------------------
# The six assertions. Each is reported on its own line. A partial pass is a
# partial pass and is never collapsed into a single boolean.
# ---------------------------------------------------------------------------

# Which run is under test. In --check mode the reference is its own subject:
# the assertions are about what the record contains, not about re-execution.
SUBJECT="$REF"
FRESH=""

if [ "$MODE" = "full" ]; then
  step "Re-executing the recorded route"
  note "This runs the model. Expect minutes, not seconds."
  for n in model tokenizer niodoo llama-cli ghost-registry; do
    check_artifact "$n" >/dev/null || fail "$EX_MISSING" "$n unavailable — run ./run fetch"
  done
  FRESH="$REPO_ROOT/runs/verify-$(date -u +%Y%m%dT%H%M%SZ)"
  say "  output: $FRESH"
  ONE_SHOT_OUT="$FRESH" \
  ONE_SHOT_MODEL="$(artifact_path model)" \
  ONE_SHOT_TOKENIZER="$(artifact_path tokenizer)" \
  ONE_SHOT_NIODOO_BIN="$(artifact_path niodoo)" \
  ONE_SHOT_LLAMA_CLI="$(artifact_path llama-cli)" \
    cargo one-shot
  rc=$?
  if [ "$rc" -ne 0 ]; then
    bad "route exited $rc before completing"
    note "output kept at $FRESH"
    exit "$EX_MISMATCH"
  fi
  SUBJECT="$FRESH"
fi

STORE="$SUBJECT/teach/self-saved-store.jsonl"
TX="$SUBJECT/transfer/r1/complete.stdout.txt"
SCORE="$SUBJECT/transfer/r1/score.json"
ORACLE="$SUBJECT/transfer/r1/oracle-gate.stdout.txt"

step "Assertions"

# 1 — the durable store holds a rule, and no digits.
if [ ! -f "$STORE" ]; then
  assert_bad "1. store present" "expected $STORE"
else
  s_sha="$(sha_of "$STORE")"
  digits="$(grep -o '[0-9]' "$STORE" | wc -l | tr -d ' ')"
  if [ "$s_sha" = "$WANT_STORE_SHA" ] && [ "$digits" -eq 0 ]; then
    assert_ok "1. store is the rule text, hash ok, contains no digits"
  elif [ "$digits" -ne 0 ]; then
    assert_bad "1. store contains $digits digit character(s)" \
               "a store holding the answer would invalidate the transfer claim"
  else
    assert_bad "1. store hash mismatch" "expected $WANT_STORE_SHA" "actual   $s_sha"
  fi
fi

# 2 — transfer stream is byte-identical to the reference.
if [ ! -f "$TX" ]; then
  assert_bad "2. transfer stream present" "expected $TX"
else
  t_sha="$(sha_of "$TX")"; t_bytes="$(size_of "$TX")"
  if [ "$t_sha" = "$WANT_TX_SHA" ] && [ "$t_bytes" = "$WANT_TX_BYTES" ]; then
    if [ "$MODE" = "full" ]; then
      assert_ok "2. transfer stream byte-identical to reference ($t_bytes bytes)"
    else
      assert_ok "2. reference transfer stream matches its manifest hash ($t_bytes bytes)"
    fi
  elif [ "$MODE" = "full" ]; then
    # The stream logs where the model, tokenizer and session file live. Run the route
    # from a different directory and those absolute paths differ, so raw byte equality
    # is only reachable at the original filesystem location. Retry with every absolute
    # path reduced to its basename, applied identically to both sides, and report how
    # many lines that accounted for. Any difference outside those lines still fails.
    REFTX="$REF/transfer/r1/complete.stdout.txt"
    basenames() { sed -E 's#/[A-Za-z0-9._+-]+(/[A-Za-z0-9._+-]+)+#<PATH>#g' "$1"; }
    n_diff="$(diff <(basenames "$REFTX") <(basenames "$TX") | grep -c '^[<>]' || true)"
    if [ "$n_diff" -eq 0 ]; then
      raw_lines="$(diff "$REFTX" "$TX" | grep -c '^[<>]' || true)"
      assert_ok "2. transfer stream identical to reference apart from logged paths"
      note "$raw_lines line(s) differ, all of them absolute paths; every other byte matches"
      note "byte-for-byte equality additionally requires running from the recorded"
      note "filesystem location. See DETERMINISM.md section 7."
    else
      assert_bad "2. transfer stream differs from reference beyond logged paths" \
                 "expected $WANT_TX_SHA ($WANT_TX_BYTES bytes)" \
                 "actual   $t_sha ($t_bytes bytes); $n_diff line(s) differ after path normalisation"
    fi
  else
    assert_bad "2. transfer stream differs from reference" \
               "expected $WANT_TX_SHA ($WANT_TX_BYTES bytes)" \
               "actual   $t_sha ($t_bytes bytes)"
  fi
fi

# 3 — the answer appears exactly once, and it is the model's final line.
if [ -f "$TX" ]; then
  # -F is required, not stylistic: the expected answer "[5, 4, 3, 2, 1, 5]" is a
  # valid regex character class, and as a pattern it would match every digit and
  # comma in the stream instead of the literal sequence.
  n="$(grep -oF -- "$GOLD" "$TX" | wc -l | tr -d ' ')"
  line="$(grep -nF -- "$GOLD" "$TX" | head -1 | cut -d: -f1)"
  last="$(wc -l < "$TX" | tr -d ' ')"
  if [ "$n" = "$GOLD_N" ] && [ "$line" = "$GOLD_LINE" ] && [ "$line" = "$last" ]; then
    assert_ok "3. answer occurs exactly once, on line $line, the final line"
  else
    assert_bad "3. answer placement wrong" \
               "expected $GOLD_N occurrence(s), on line $GOLD_LINE of $GOLD_LINE" \
               "actual   $n occurrence(s), first on line ${line:-none} of $last"
  fi
fi

# 4 — the authoritative scorer.
if [ ! -f "$SCORE" ]; then
  assert_bad "4. score file present" "expected $SCORE"
else
  st="$(python3 -c "import json,sys;print(json.load(open(sys.argv[1])).get('status',''))" "$SCORE" 2>/dev/null)"
  ex="$(python3 -c "import json,sys;print(json.load(open(sys.argv[1])).get('exact_answer',''))" "$SCORE" 2>/dev/null)"
  bn="$(python3 -c "import json,sys;print(json.load(open(sys.argv[1])).get('banned_hits',''))" "$SCORE" 2>/dev/null)"
  if [ "$st" = "$WANT_STATUS" ] && [ "$ex" = "True" ] && [ "$bn" = "[]" ]; then
    assert_ok "4. score $st, exact answer, no banned words"
  else
    assert_bad "4. score did not pass" "expected $WANT_STATUS / exact / []" "actual   $st / $ex / $bn"
  fi
fi

# 5 — the excluded-path oracle gate.
if [ ! -f "$ORACLE" ]; then
  assert_bad "5. oracle gate present" "expected $ORACLE"
else
  g="$(tr -d '[:space:]' < "$ORACLE")"
  if [ "$g" = "$WANT_ORACLE" ]; then
    assert_ok "5. oracle gate $WANT_ORACLE"
  else
    assert_bad "5. oracle gate not OK" "expected $WANT_ORACLE" "actual   $g"
  fi
fi

# 6 — the control was asked the same question as the treatment. This is the
# structural claim: the only difference between the failure and the success is
# the runtime path, not the prompt.
CTRL="$SUBJECT/control/wording/prompt.txt"
DEST="$REPO_ROOT/flag/session.txt"
if [ ! -f "$CTRL" ] || [ ! -f "$DEST" ]; then
  assert_bad "6. control and destination prompts present" "control: $CTRL" "destination: $DEST"
else
  a="$(tr -d '[:space:]' < "$CTRL" | sha256sum | cut -d' ' -f1)"
  b="$(tr -d '[:space:]' < "$DEST" | sha256sum | cut -d' ' -f1)"
  if [ "$a" = "$b" ]; then
    assert_ok "6. control prompt is identical to the destination prompt"
  else
    assert_bad "6. control and destination prompts differ" "control $a" "destination $b"
  fi
fi

# ---------------------------------------------------------------------------
step "Result"
say "  $pass_n of $((pass_n + fail_n)) assertions passed"

if [ "$fail_n" -eq 0 ]; then
  if [ "$MODE" = "check" ]; then
    ok "the recorded run satisfies every claim made about it"
    note "This checked the record. To re-execute it here: ./run verify"
    note "To reproduce the order effect on your own hardware: ./run sweep"
  else
    ok "this machine reproduced the recorded run"
    note "output: $FRESH"
  fi
  exit "$EX_OK"
fi

bad "$fail_n assertion(s) failed"
[ -n "$FRESH" ] && note "output kept for inspection at $FRESH"
exit "$EX_MISMATCH"
