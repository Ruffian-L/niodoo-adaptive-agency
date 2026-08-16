#!/usr/bin/env bash
# docs-check — fail closed on documentation drift. Offline; no model or GPU.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

[ "$#" -eq 0 ] || fail "$EX_ENV" "docs-check takes no arguments"

fail_n=0
pass_n=0

pass() { ok "$1"; pass_n=$((pass_n + 1)); }
miss() { bad "$1"; fail_n=$((fail_n + 1)); }

contains() {
  local file="$1" text_value="$2" label="$3"
  if grep -Fq -- "$text_value" "$REPO_ROOT/$file"; then
    pass "$label"
  else
    miss "$label — missing from $file"
  fi
}

absent() {
  local text_value="$1" label="$2"
  shift 2
  if grep -nF -- "$text_value" "$@" >/dev/null 2>&1; then
    miss "$label"
    grep -nF -- "$text_value" "$@" >&2 || true
  else
    pass "$label"
  fi
}

say "niodoo-adaptive-agency — docs-check"
step "Canonical title and claim"

contains PAPER.md "# Knowing Where You Are" "paper title"
contains PAPER.md "## Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo" "paper subtitle"
contains README.md "## Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo" "README title"
contains .zenodo.json '"title": "Knowing Where You Are: Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo"' "Zenodo title"
contains CITATION.cff 'title: "Knowing Where You Are: Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo"' "citation title"
contains publication/HUGGING_FACE_DATASET_CARD.md 'pretty_name: "Knowing Where You Are: Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo"' "dataset title"
contains PAPER.md "usable knowledge of where a system is in its current" "operational-consciousness definition"
contains FALSIFIERS.md "No phenomenal, biological, or human-consciousness claim" "phenomenal-consciousness boundary"

step "Seal, durability, and authorship contract"

for file in README.md RECORD.md docs/seal/SEAL_20260814.md docs/climb/LANGUAGE.md; do
  contains "$file" "Grok Seal" "$file names Grok Seal"
  contains "$file" "Jason & Sol Seal" "$file names Jason & Sol Seal"
done
contains docs/seal/SEAL_20260814.md "not ownership" "seal names reject ownership"
contains README.md "not a third seal" "durability is not a third seal"
contains SWEEP.md "56 additional real memories" "56-memory condition is explicit"
contains SWEEP.md "Two passes in six arrangements are not a general reliability estimate" "durability rate boundary"
contains CREDITS.md "sole author of this project and paper" "Jason sole authorship"
contains CREDITS.md "The roster is non-hierarchical" "collaborator peers are non-hierarchical"

current_docs=(
  "$REPO_ROOT/README.md"
  "$REPO_ROOT/RECORD.md"
  "$REPO_ROOT/FALSIFIERS.md"
  "$REPO_ROOT/docs/seal/SEAL_20260814.md"
  "$REPO_ROOT/docs/seal/SELECTED_MOMENTS.md"
)
absent "No consciousness claim" "stale unconditional consciousness denial" "${current_docs[@]}"
absent "not a declaration of consciousness" "stale consciousness boundary" "${current_docs[@]}"

step "Historical status banners"

contains docs/process/MERGED_PREFLIGHT.md "Historical status note (2026-08-16)" "merged preflight banner"
contains docs/process/ONE_SHOT.md "Historical route note (2026-08-16)" "one-shot banner"
contains docs/climb/CURRENT_COORDINATE.md "Historical coordinate note (2026-08-16)" "coordinate banner"
contains docs/climb/CLIMB_MAP.md "Historical map note (2026-08-16)" "climb-map banner"
contains docs/climb/MILESTONES.md "Historical milestone note (2026-08-16)" "milestone banner"
contains docs/climb/CLIMB_CARD.md "Historical climb note (2026-08-16)" "climb-card banner"
contains docs/seal/FLAGGED_DESTINATION.md "Historical flag note (2026-08-16)" "flagged-destination banner"

step "Metadata structure"

if python3 - "$REPO_ROOT/.zenodo.json" <<'PY'
import json, sys
data = json.load(open(sys.argv[1], encoding="utf-8"))
assert data["creators"] == [{"name": "Pham, Jason Van"}]
PY
then
  pass "Zenodo has one administrative creator: Jason Van Pham"
else
  miss "Zenodo creator metadata"
fi

if [ "$(grep -c '^  - family-names:' "$REPO_ROOT/CITATION.cff")" -eq 1 ] &&
   grep -Fq 'family-names: "Pham"' "$REPO_ROOT/CITATION.cff"; then
  pass "citation has one author: Jason Van Pham"
else
  miss "citation author metadata"
fi

step "Evidence packs"

for dir in \
  "$REPO_ROOT/evidence/history/july-gravity-20260718" \
  "$REPO_ROOT/evidence/durability/20260815"; do
  if [ -f "$dir/SHA256SUMS" ] && (cd "$dir" && sha256sum -c SHA256SUMS >/dev/null); then
    pass "$(realpath --relative-to="$REPO_ROOT" "$dir") hashes"
  else
    miss "$(realpath --relative-to="$REPO_ROOT" "$dir") hashes"
  fi
done

if python3 - "$REPO_ROOT/evidence/durability/20260815/results.csv" <<'PY'
import csv, sys
rows = list(csv.DictReader(open(sys.argv[1], newline="", encoding="utf-8")))
shuffles = [r for r in rows if r["phase"] == "shuffle" and r["added_memories"] == "56"]
assert len(shuffles) == 6
assert sum(r["status"] == "PASS" for r in shuffles) == 2
assert sum(r["status"] == "FAIL" for r in shuffles) == 4
assert len({r["stdout_sha256"] for r in shuffles if r["status"] == "PASS"}) == 1
PY
then
  pass "durability table encodes 2/6 with byte-identical passes"
else
  miss "durability table invariants"
fi

if grep -R -nE '/home/|/Users/' "$REPO_ROOT/evidence/history" "$REPO_ROOT/evidence/durability" >/dev/null 2>&1; then
  miss "evidence packs contain absolute personal paths"
else
  pass "evidence packs contain no absolute personal paths"
fi

step "Internal Markdown links"

if python3 - "$REPO_ROOT" <<'PY'
import pathlib, re, sys

root = pathlib.Path(sys.argv[1]).resolve()
roots = [root, root / "docs", root / "publication", root / "evidence", root / "reference"]
files = set(root.glob("*.md"))
for directory in roots[1:]:
    if directory.exists():
        files.update(directory.rglob("*.md"))

bad = []
pattern = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
for source in sorted(files):
    text = source.read_text(encoding="utf-8", errors="replace")
    for raw in pattern.findall(text):
        target = raw.strip()
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        target = target.split("#", 1)[0]
        if not target or target.startswith(("http://", "https://", "mailto:")):
            continue
        target = target.split(" ", 1)[0]
        if "*" in target or "…" in target:
            continue
        resolved = (source.parent / target).resolve()
        if not resolved.exists():
            bad.append(f"{source.relative_to(root)} -> {raw}")

if bad:
    print("\n".join(bad), file=sys.stderr)
    raise SystemExit(1)
PY
then
  pass "internal Markdown link targets exist"
else
  miss "broken internal Markdown links"
fi

step "Result"
say "  $pass_n checks passed; $fail_n failed"

if [ "$fail_n" -eq 0 ]; then
  ok "documentation contract holds"
  exit "$EX_OK"
fi

bad "documentation contract failed"
exit "$EX_MISMATCH"
