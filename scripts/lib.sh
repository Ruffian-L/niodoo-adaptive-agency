# scripts/lib.sh — shared helpers. Sourced, never executed.
#
# Output rules for every command in this release:
#   - say what is about to happen, and the expected hash, before doing it
#   - never silently succeed on a skip; say "already present, hash ok"
#   - a failure names the artifact, the expected value and the actual value
#   - never truncate, collapse or hide output behind a spinner

: "${REPO_ROOT:?lib.sh requires REPO_ROOT}"

# Exit codes are a contract. See ./run.
readonly EX_OK=0
readonly EX_MISMATCH=1
readonly EX_ENV=2
readonly EX_MISSING=3

MANIFEST_PY="$REPO_ROOT/scripts/manifest.py"

# Optional local overrides. Gitignored, so machine-specific absolute paths never
# enter the repository. See .env.local.example. This is also how an artifact with
# no published URL is supplied.
#
# Precedence is explicit environment > .env.local > manifest dest. Sourcing the file
# directly would invert the first two and silently ignore a variable the caller set
# on the command line, so each assignment is applied only if the name is still unset.
if [ -f "$REPO_ROOT/.env.local" ]; then
  while IFS= read -r _line; do
    case "$_line" in ''|'#'*) continue ;; esac
    [ "${_line#*=}" = "$_line" ] && continue      # no '=' on the line
    _k="${_line%%=*}"; _v="${_line#*=}"
    _k="${_k#"${_k%%[![:space:]]*}"}"; _k="${_k%"${_k##*[![:space:]]}"}"
    case "$_k" in ''|*[!A-Za-z0-9_]*) continue ;; esac
    _v="${_v%\"}"; _v="${_v#\"}"; _v="${_v%\'}"; _v="${_v#\'}"
    [ -n "${!_k+set}" ] || export "$_k=$_v"
  done < "$REPO_ROOT/.env.local"
  unset _line _k _v
fi

# Colour only when attached to a terminal, so piped output stays clean for CI.
if [ -t 1 ]; then
  C_OK=$'\033[32m'; C_BAD=$'\033[31m'; C_WARN=$'\033[33m'; C_DIM=$'\033[2m'; C_OFF=$'\033[0m'
else
  C_OK=''; C_BAD=''; C_WARN=''; C_DIM=''; C_OFF=''
fi

say()  { printf '%s\n' "$*"; }
step() { printf '\n%s\n' "$*"; }
ok()   { printf '  %sok%s        %s\n' "$C_OK" "$C_OFF" "$*"; }
bad()  { printf '  %sFAILED%s    %s\n' "$C_BAD" "$C_OFF" "$*"; }
warn() { printf '  %swarn%s      %s\n' "$C_WARN" "$C_OFF" "$*"; }
note() { printf '  %s%s%s\n' "$C_DIM" "$*" "$C_OFF"; }

# fail <exit-code> <message...>
fail() { local code="$1"; shift; bad "$*"; exit "$code"; }

# mf <manifest.py args...>
mf() { python3 "$MANIFEST_PY" "$@"; }

# artifact_path <name>
# Resolution order: the artifact's env_override if set and present, then the
# manifest dest under vendor/. Printing the resolved path is the caller's job.
artifact_path() {
  local name="$1" envvar override suffix dest
  envvar="$(mf env "$name")"
  if [ -n "$envvar" ]; then
    override="${!envvar:-}"
    if [ -n "$override" ]; then
      # Some overrides name a root directory rather than the file itself.
      suffix="$(mf get "$name" env_override_suffix)"
      if [ -d "$override" ] && [ -n "$suffix" ]; then
        override="${override%/}/$suffix"
      fi
      # A directory is valid only for artifacts that extract to a tree. For every
      # other artifact a directory means the override points at the wrong thing.
      if [ -f "$override" ] || { [ -d "$override" ] && [ -n "$(mf get "$name" extract_to)" ]; }; then
        printf '%s\n' "$override"; return 0
      fi
    fi
  fi
  dest="$(mf get "$name" dest)"
  printf '%s\n' "$REPO_ROOT/$dest"
}

# artifact_usable_path <name>
# The path a CONSUMER should read. For artifacts that unpack, that is the extracted
# tree, not the archive: artifact_path returns the download destination, which for the
# reference run is a .tar.zst. Resolving a reader to the archive is why the clean
# checkout could fetch successfully and then fail to verify.
artifact_usable_path() {
  local name="$1" envvar override extract
  envvar="$(mf env "$name")"
  if [ -n "$envvar" ]; then
    override="${!envvar:-}"
    [ -n "$override" ] && [ -e "$override" ] && { printf '%s\n' "$override"; return 0; }
  fi
  extract="$(mf get "$name" extract_to)"
  if [ -n "$extract" ] && [ -d "$REPO_ROOT/$extract" ]; then
    printf '%s\n' "$REPO_ROOT/$extract"; return 0
  fi
  artifact_path "$name"
}

# sha_of <path>
sha_of() { sha256sum "$1" 2>/dev/null | cut -d' ' -f1; }

# size_of <path>
# -L dereferences, matching sha256sum. Without it a symlinked artifact reports the
# length of the link target string and disagrees with its own hash.
size_of() { stat -Lc%s "$1" 2>/dev/null || echo 0; }

# check_artifact <name>
# Verifies presence, size and hash. Reports each result on its own line.
# Returns 0 ok, 3 missing or mismatched.
check_artifact() {
  local name="$1" path want_sha want_bytes got_sha got_bytes
  path="$(artifact_path "$name")"
  want_sha="$(mf get "$name" sha256)"
  want_bytes="$(mf get "$name" bytes)"

  if [ ! -e "$path" ]; then
    bad "$name — not present"
    note "expected at $path"
    [ -n "$(mf env "$name")" ] && note "or set $(mf env "$name")=<path>"
    return "$EX_MISSING"
  fi

  # Extracted trees are verified by their inner files, listed in the manifest as
  # [[artifact.contains]]. A directory has no meaningful size or hash of its own.
  if [ -d "$path" ]; then
    check_extracted_tree "$name" "$path"
    return $?
  fi

  got_bytes="$(size_of "$path")"
  if [ -n "$want_bytes" ] && [ "$want_bytes" != "0" ] && [ "$got_bytes" != "$want_bytes" ]; then
    bad "$name — wrong size"
    note "expected $want_bytes bytes"
    note "actual   $got_bytes bytes"
    return "$EX_MISSING"
  fi

  if [ -n "$want_sha" ]; then
    got_sha="$(sha_of "$path")"
    if [ "$got_sha" != "$want_sha" ]; then
      bad "$name — hash mismatch"
      note "expected $want_sha"
      note "actual   $got_sha"
      note "at       $path"
      return "$EX_MISSING"
    fi
    ok "$name — already present, hash ok"
    note "$path"
    return "$EX_OK"
  fi

  ok "$name — present (no hash pinned)"
  note "$path"
  return "$EX_OK"
}

# check_extracted_tree <name> <dir>
# Verifies the [[artifact.contains]] entries inside an extracted artifact tree.
# Each inner file is reported on its own line; a truncated download must fail
# loudly rather than diff against a file that is not there.
check_extracted_tree() {
  local name="$1" root="$2" rc="$EX_OK" rel want_sha want_bytes got_sha got_bytes full n=0
  while IFS=$'\t' read -r rel want_sha want_bytes; do
    [ -z "$rel" ] && continue
    n=$((n+1))
    full="$root/$rel"
    if [ ! -f "$full" ]; then
      bad "$name — missing $rel"; note "expected at $full"; rc="$EX_MISSING"; continue
    fi
    got_bytes="$(size_of "$full")"
    if [ "$want_bytes" != "0" ] && [ "$got_bytes" != "$want_bytes" ]; then
      bad "$name — $rel wrong size"
      note "expected $want_bytes bytes"; note "actual   $got_bytes bytes"
      rc="$EX_MISSING"; continue
    fi
    got_sha="$(sha_of "$full")"
    if [ "$got_sha" != "$want_sha" ]; then
      bad "$name — $rel hash mismatch"
      note "expected $want_sha"; note "actual   $got_sha"
      rc="$EX_MISSING"; continue
    fi
    ok "$name/$rel — hash ok"
  done < <(mf contains "$name")

  if [ "$n" -eq 0 ]; then
    ok "$name — present (tree, no inner files pinned)"
    note "$root"
  elif [ "$rc" = "$EX_OK" ]; then
    note "$root"
  fi
  return "$rc"
}

# require_cmd <binary> <hint>
require_cmd() {
  if command -v "$1" >/dev/null 2>&1; then
    ok "$1 — $(command -v "$1")"
    return 0
  fi
  bad "$1 not found on PATH"
  [ -n "${2:-}" ] && note "$2"
  return 1
}
