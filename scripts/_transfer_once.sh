#!/usr/bin/env bash
# _transfer_once.sh — internal. One transfer against one store.
#
# Used by sweep, which needs the transfer phase alone rather than the full route:
# the control lane and the teaching phase are identical across arrangements, so
# re-running them would multiply the cost without changing anything measured.
#
# The runtime configuration below is copied from the sealed flag environment. Only
# --remember-store varies between calls.
#
#   _transfer_once.sh <store> <outdir>

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

STORE="${1:?store path required}"
OUT="${2:?output directory required}"
DEST="${ONE_SHOT_DESTINATION:-$REPO_ROOT/flag/session.txt}"
mkdir -p "$OUT"

NIODOO="$(artifact_path niodoo)"
MODEL="$(artifact_path model)"
TOKENIZER="$(artifact_path tokenizer)"
REGISTRY="$(artifact_path ghost-registry)"
# The engine locates the registry relative to a product root, five levels up from
# niodv4/data/results/summaries/ghost_candidate_registry.json.
PRODUCT_ROOT="${ONE_SHOT_PRODUCT_ROOT:-$(cd "$(dirname "$REGISTRY")/../../../.." && pwd)}"
SOURCE_ROOT="${ONE_SHOT_SOURCE_ROOT:-$REPO_ROOT}"

cd "$SOURCE_ROOT" || exit "$EX_ENV"

# Sealed flag environment. NIODOO_HARD_CLAIM is assembled from fragments in the
# route source, so a grep for the literal name will not find it there.
NIODOO_HARD_CLAIM=1 \
NIODOO_GOD_ZONE_RECOVERY=1 \
NIODOO_REMEMBER_EAR_PIN_GOAL=1 \
NIODOO_STRUCTURAL_GOAL=0 \
NIODOO_REPO_ROOT="$PRODUCT_ROOT" \
NIODOO_REMEMBER_RESIDUAL_EARS=1 \
NIODOO_DUAL_STREAM=1 \
NIODOO_DUAL_INJECT_GAIN=1.0 \
NIODOO_DUAL_POSTURE_BOOST=8 \
NIODOO_REMEMBER_EAR_MASS=5 \
NIODOO_REMEMBER_EAR_BETA=1.0 \
NIODOO_REMEMBER_EAR_LOGIT_BOOST=1.2 \
NIODOO_REMEMBER_EAR_ORDER_BOOST=0 \
NIODOO_REMEMBER_EAR_STOP_BOOST=0 \
NIODOO_REMEMBER_FORCE_ALL= \
NIODOO_REMEMBER_PROC_ENABLE= \
NIODOO_REMEMBER_EAR_PROGRESS= \
NIODOO_STRUCTURAL_GOAL_PHRASE= \
"$NIODOO" \
  --model-path "$MODEL" \
  --model-arch auto \
  --tokenizer-path "$TOKENIZER" \
  --chat-template auto --system-prompt-mode free \
  --output-contract-mode off --context-length 8192 \
  --max-steps 768 --temperature 0.0 \
  --theta-override 1.5 --physics-blend 0.9 \
  --physics-start-layer 16 --physics-end-layer 33 \
  --ablate-periodic-controller --ablate-live-motifs \
  --workspace-tools false \
  --remember-store "$STORE" \
  --session-script "$DEST" \
  --telemetry-out "$OUT/telemetry.jsonl" \
  --telemetry-profile full --stdout-profile telemetry \
  > "$OUT/complete.stdout.txt" 2> "$OUT/complete.stderr.txt"
