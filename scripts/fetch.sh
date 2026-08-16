#!/usr/bin/env bash
# fetch — download every manifest artifact and verify its hash.
#
# Idempotent by hash, never by timestamp or marker file. Resumable: partial
# downloads continue rather than restart. Verification and use are separate
# commands specifically so `./run verify` can run with no network at all.
#
# Exit 0 all present and verified, 3 if anything is missing or fails its hash.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

only_lane=""
while [ $# -gt 0 ]; do
  case "$1" in
    --lane) only_lane="${2:-}"; shift 2 ;;
    -h|--help) say "usage: ./run fetch [--lane verify|teach|chat|sweep]"; exit 0 ;;
    *) fail "$EX_ENV" "unknown option: $1" ;;
  esac
done

say "niodoo-adaptive-agency — fetch"
[ -n "$only_lane" ] && say "lane: $only_lane"
note "Hashes come from manifest.toml. Nothing else in this repo hardcodes a URL or hash."

failed=0
unpublished=()

if [ -n "$only_lane" ]; then
  names="$(mf list --lane "$only_lane")"
else
  names="$(mf list)"
fi

for name in $names; do
  step "$name — $(mf get "$name" description)"

  path="$(artifact_path "$name")"
  want_sha="$(mf get "$name" sha256)"
  want_bytes="$(mf get "$name" bytes)"
  url="$(mf get "$name" url)"

  # Say what is about to happen, and the expected hash, before doing it.
  [ -n "$want_sha" ]   && note "expect sha256 $want_sha"
  [ -n "$want_bytes" ] && [ "$want_bytes" != "0" ] && note "expect $want_bytes bytes"

  if check_artifact "$name"; then
    continue
  fi

  if [ -z "$url" ]; then
    bad "$name — not present and no published URL"
    note "This artifact is not yet available for download."
    note "Supply a local copy with $(mf env "$name")=<path> in .env.local"
    unpublished+=("$name")
    failed=$((failed+1))
    continue
  fi

  mkdir -p "$(dirname "$path")"
  say "  downloading from $url"
  # -C - resumes a partial file; --fail turns an HTTP error into a non-zero exit
  # rather than a saved error page that would then fail its hash confusingly.
  if ! curl --proto '=https' --tlsv1.2 --fail --location --retry 3 -C - \
            -o "$path" "$url"; then
    bad "$name — download failed"
    failed=$((failed+1))
    continue
  fi

  if ! check_artifact "$name"; then
    bad "$name — downloaded but failed verification; leaving the file in place for inspection"
    failed=$((failed+1))
    continue
  fi

  if [ "$(mf get "$name" executable)" = "true" ]; then
    chmod +x "$path" && note "marked executable"
  fi

  extract_to="$(mf get "$name" extract_to)"
  if [ -n "$extract_to" ]; then
    dest="$REPO_ROOT/$extract_to"
    say "  extracting to $dest"
    mkdir -p "$dest"
    if tar --extract --file "$path" --directory "$dest" --strip-components=1; then
      ok "$name — extracted"
      check_extracted_tree "$name" "$dest" || failed=$((failed+1))
    else
      bad "$name — extraction failed"
      failed=$((failed+1))
    fi
  fi
done

step "Summary"
if [ "$failed" -eq 0 ]; then
  ok "every artifact present and verified"
  note "next: ./run verify"
  exit "$EX_OK"
fi

bad "$failed artifact(s) unavailable"
if [ "${#unpublished[@]}" -gt 0 ]; then
  note "not yet published: ${unpublished[*]}"
  note "copy .env.local.example to .env.local and point at local copies"
fi
exit "$EX_MISSING"
