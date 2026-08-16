# Setup

Everything runs through `./run`. `./run` with no argument lists the subcommands.

```bash
./run doctor          # what this machine can run. Downloads nothing, writes nothing.
./run verify --check  # assert the record. Seconds, no GPU, no model, no network.
./run docs-check      # check titles, claims, attribution, links, and evidence hashes.
./run install         # build the repo tooling
./run fetch           # download and hash-verify every artifact
./run verify          # re-execute the recorded route and diff against it
./run chat            # REPL with the durable store enabled
./run sweep           # reshuffle store order and measure the pass rate
```

## Two lanes, different requirements

**Reading the record** needs nothing. `RECORD.md`, `FALSIFIERS.md`, `DETERMINISM.md`,
`reference/` and the recorded outputs under `flag/`, `coordinates/` and `evidence/`
are plain text. `./run verify --check` asserts the central six sealed-route claims;
`./run docs-check` checks the release's documentation contract. Both run on any
machine in seconds.

**Running inference** needs the exact lane:

| | |
|---|---|
| OS / arch | Linux `aarch64` |
| GPU | NVIDIA GB10-class |
| CUDA | 13 |
| Libraries | `libcuda`, `libnvrtc`, `libcurand`, `libcublas`, `libcublasLt`, `libcudart` |

`./run doctor` checks each and names whatever is missing. The pinned binary is not
cross-platform.

## Artifacts

`manifest.toml` is the only file in the repository that records a URL, hash or size.
Every script reads from it, so changing an artifact means changing one file.

`./run fetch` downloads the model and tokenizer from upstream and verifies both
against their pinned hashes. It is resumable and idempotent by hash — re-running it
reports `already present, hash ok` rather than re-downloading.

### Artifacts without a published URL

The Niodoo runtime, the `llama-cli` build and the ghost-basin registry are release
assets. Until they are published, supply local copies:

```bash
cp .env.local.example .env.local     # then edit
```

`.env.local` is gitignored, so machine-specific paths never enter the repository. An
override is verified against the same hash as a download; a local file that fails its
hash is rejected exactly like a bad fetch.

Precedence is: explicit environment variable, then `.env.local`, then the manifest
destination under `vendor/`.

## Building from source

`./run install` builds the tooling in this repository — the map checker and the route
driver. **It does not build the inference engine.**

The engine ships as a hash-pinned binary because a clean build at the recorded product
revision does not compile: it references cache and hook code that was untracked at
that commit. Reproduction here is binary-level. `DETERMINISM.md` §2 states the
boundary and what it would take to move it.

## Offline use

`fetch` and `verify` are separate commands specifically so `verify` can run with no
network at all. Nothing in the verification path contacts a remote host, and there is
no telemetry, version check or auto-update anywhere in this repository.
