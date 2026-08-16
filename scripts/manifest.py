#!/usr/bin/env python3
"""Read manifest.toml and emit shell-consumable output.

manifest.toml is the only place an artifact URL, hash, size or destination is
written. Every script queries it through this helper rather than hardcoding
anything.

Usage:
  manifest.py list [--lane LANE]     one artifact name per line
  manifest.py get NAME FIELD         one field value, empty string if unset
  manifest.py expected FIELD         a value from the [expected] table
  manifest.py meta FIELD             a value from the [meta] table
  manifest.py cuda-libs              one library stem per line
  manifest.py contains NAME          "path<TAB>sha256<TAB>bytes" per inner file
  manifest.py env NAME               the artifact's env_override variable name
"""

import sys
import tomllib
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
MANIFEST = REPO_ROOT / "manifest.toml"


def load():
    if not MANIFEST.exists():
        sys.stderr.write(f"manifest not found: {MANIFEST}\n")
        raise SystemExit(2)
    with MANIFEST.open("rb") as fh:
        return tomllib.load(fh)


def artifact(doc, name):
    for a in doc.get("artifact", []):
        if a.get("name") == name:
            return a
    sys.stderr.write(f"no artifact named {name!r} in manifest.toml\n")
    raise SystemExit(2)


def main(argv):
    if not argv:
        sys.stderr.write(__doc__)
        return 2
    doc = load()
    cmd, rest = argv[0], argv[1:]

    if cmd == "list":
        lane = None
        if len(rest) >= 2 and rest[0] == "--lane":
            lane = rest[1]
        for a in doc.get("artifact", []):
            if lane is None or lane in a.get("required_for", []):
                print(a["name"])
        return 0

    if cmd == "get":
        if len(rest) != 2:
            sys.stderr.write("usage: manifest.py get NAME FIELD\n")
            return 2
        value = artifact(doc, rest[0]).get(rest[1], "")
        # bools print as lowercase so `[ "$v" = true ]` works in shell
        if isinstance(value, bool):
            value = "true" if value else "false"
        print(value)
        return 0

    if cmd == "env":
        print(artifact(doc, rest[0]).get("env_override", ""))
        return 0

    if cmd == "expected":
        value = doc.get("expected", {}).get(rest[0], "")
        if isinstance(value, bool):
            value = "true" if value else "false"
        print(value)
        return 0

    if cmd == "meta":
        print(doc.get("meta", {}).get(rest[0], ""))
        return 0

    if cmd == "cuda-libs":
        for lib in doc.get("meta", {}).get("cuda_libs", []):
            print(lib)
        return 0

    if cmd == "contains":
        for entry in artifact(doc, rest[0]).get("contains", []):
            print(f"{entry.get('path','')}\t{entry.get('sha256','')}\t{entry.get('bytes',0)}")
        return 0

    sys.stderr.write(f"unknown command: {cmd}\n")
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
