#!/usr/bin/env bash
# doctor — preflight. Reports what this machine can and cannot run.
# Downloads nothing and writes nothing. Safe to run first, always.
#
# Exit 0 if every lane is available, 2 if the environment blocks one.

set -uo pipefail
: "${REPO_ROOT:=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
. "$REPO_ROOT/scripts/lib.sh"

problems=0
blocked_lanes=()

say "niodoo-adaptive-agency — preflight"
say "repo: $REPO_ROOT"

# ---------------------------------------------------------------- platform
step "Platform"
os="$(uname -s)"; arch="$(uname -m)"
want_platform="$(mf meta platform)"
say "  required: $want_platform"
say "  detected: $(printf '%s-%s' "$os" "$arch" | tr '[:upper:]' '[:lower:]')"

if [ "$os" = "Linux" ] && [ "$arch" = "aarch64" ]; then
  ok "platform matches the recorded lane"
else
  bad "platform does not match the recorded lane"
  note "The pinned binary is not cross-platform. verify/teach/chat/sweep cannot run here."
  note "The record itself is still readable: RECORD.md, FALSIFIERS.md, reference/."
  problems=$((problems+1)); blocked_lanes+=("verify" "teach" "chat" "sweep")
fi

# ---------------------------------------------------------------- toolchain
step "Toolchain"
require_cmd sha256sum "install coreutils" || { problems=$((problems+1)); }
require_cmd curl      "install curl"      || { problems=$((problems+1)); }
require_cmd python3   "python 3.11+ required for manifest parsing" || { problems=$((problems+1)); }
if command -v python3 >/dev/null 2>&1; then
  if python3 -c 'import tomllib' 2>/dev/null; then
    ok "python tomllib available ($(python3 -c 'import sys;print(".".join(map(str,sys.version_info[:3])))'))"
  else
    bad "python3 lacks tomllib — need python 3.11 or newer"
    problems=$((problems+1))
  fi
fi
if command -v cargo >/dev/null 2>&1; then
  ok "cargo — $(cargo --version 2>/dev/null | head -1)"
else
  warn "cargo not found — only needed for ./run install and the offline map check"
fi

# ---------------------------------------------------------------- gpu / cuda
step "GPU and CUDA"
if command -v nvidia-smi >/dev/null 2>&1; then
  gpu="$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)"
  if [ -n "$gpu" ]; then
    ok "GPU — $gpu"
    note "required class: $(mf meta gpu)"
  else
    bad "nvidia-smi present but reported no GPU"
    problems=$((problems+1)); blocked_lanes+=("verify" "teach" "chat" "sweep")
  fi
else
  bad "nvidia-smi not found"
  note "The runtime requires CUDA. Inference lanes cannot run."
  problems=$((problems+1)); blocked_lanes+=("verify" "teach" "chat" "sweep")
fi

# Snapshot the loader cache once, then match in-process with bash string containment.
# Do NOT pipe the cache into `grep -q`: grep exits at the first match, the writer takes
# SIGPIPE, and `set -o pipefail` reports that as failure. Whether it fires depends on
# where the library sits in ~1300 lines of output, so it presents as one library
# randomly reported missing. No pipe, no race.
ldcache="$(ldconfig -p 2>/dev/null || true)"
missing_libs=()
while read -r lib; do
  [ -z "$lib" ] && continue
  if [[ "$ldcache" == *"${lib}.so"* ]]; then
    ok "$lib"
  else
    bad "$lib not resolvable"
    missing_libs+=("$lib")
  fi
done < <(mf cuda-libs)
if [ "${#missing_libs[@]}" -gt 0 ]; then
  note "missing: ${missing_libs[*]}"
  problems=$((problems+1)); blocked_lanes+=("verify" "teach" "chat" "sweep")
fi

# ---------------------------------------------------------------- resources
step "Resources"
need_bytes=0
while read -r name; do
  [ -z "$name" ] && continue
  b="$(mf get "$name" bytes)"; [ -z "$b" ] && b=0
  need_bytes=$((need_bytes + b))
done < <(mf list)
need_gib=$(( (need_bytes + 1073741823) / 1073741824 ))

avail_kib="$(df -Pk "$REPO_ROOT" | awk 'NR==2{print $4}')"
avail_gib=$(( avail_kib / 1048576 ))
say "  artifacts total: ~${need_gib} GiB"
say "  free on this filesystem: ${avail_gib} GiB"
if [ "$avail_gib" -ge $((need_gib + 5)) ]; then
  ok "disk headroom sufficient"
else
  bad "insufficient disk — want at least $((need_gib + 5)) GiB free"
  problems=$((problems+1))
fi

ram_gib=$(( $(awk '/MemTotal/{print $2}' /proc/meminfo) / 1048576 ))
say "  RAM: ${ram_gib} GiB"
if [ "$ram_gib" -ge 16 ]; then
  ok "RAM sufficient"
else
  warn "under 16 GiB — the 8B at Q5_K_M may not load"
fi

# ---------------------------------------------------------------- artifacts
step "Artifacts"
note "Presence only. ./run fetch downloads and verifies; this never touches the network."
missing=0
while read -r name; do
  [ -z "$name" ] && continue
  p="$(artifact_path "$name")"
  if [ -e "$p" ]; then
    ok "$name — present"
  else
    url="$(mf get "$name" url)"
    if [ -n "$url" ]; then
      note "$name — absent, fetchable"
    else
      warn "$name — absent, and no URL is published yet"
      note "supply it with $(mf env "$name")=<path>"
    fi
    missing=$((missing+1))
  fi
done < <(mf list)

# ---------------------------------------------------------------- summary
step "Summary"
if [ "$problems" -eq 0 ]; then
  ok "environment supports every lane"
  [ "$missing" -gt 0 ] && note "$missing artifact(s) still to fetch — run: ./run fetch"
  exit "$EX_OK"
fi

bad "$problems environment problem(s)"
if [ "${#blocked_lanes[@]}" -gt 0 ]; then
  uniq_lanes="$(printf '%s\n' "${blocked_lanes[@]}" | sort -u | tr '\n' ' ')"
  note "blocked lanes: $uniq_lanes"
fi
note "The record can still be read and audited without any of this."
exit "$EX_ENV"
