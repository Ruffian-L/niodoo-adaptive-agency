# Determinism contract

Every setting the byte-identity claim depends on. `./run verify` asserts these before
running and refuses with exit code `2`, naming the one that differs.

If you change anything on this page, you are producing a new coordinate. That is a
legitimate thing to do. It is not a reproduction of the recorded run and must not be
reported as one.

---

## 1. Byte identities

| Artifact | SHA-256 | Bytes |
|---|---|---|
| `Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf` | `14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7` | 5,732,992,416 |
| Llama 3.1 tokenizer | `79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4` | 9,085,657 |
| Niodoo runtime | `2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507` | 90,104,160 |
| `llama-cli` (build `c0bc859`) | `72c08ab8827be6004f3d5435064203f2cbadc552f6bcb6dfd1ada146624b4b5d` | 74,261,680 |
| Ghost-basin registry | `6e361f83c24d7d2d0b3534279a4f410761793cf774fac6c3d4ae64c77cfa747b` | 24,138 |

All five live in `manifest.toml`, which is the only file in this repository that
records a hash, size or URL.

## 2. Reproduction is binary-level, not source-level

**Stated plainly because it matters more than it is convenient.**

The Niodoo runtime ships as a hash-pinned binary. A clean detached build at the
recorded product revision `9de966d` does not compile: it references cache and hook
code that was untracked or absent at that commit. The binary hash is therefore the
executable identity, and the recorded revision is an incomplete source coordinate.

What this release supports:

- **Verified**: this exact binary, on this platform, produces the recorded bytes.
- **Not supported**: rebuilding that binary from source and getting the same bytes.

A `verify --from-source` lane — build current source, expect different bytes, assert
the same final answer — is the natural next milestone. It is not in this release
because the source state it would build has not been reconstructed and frozen.

The engine source is not published. The compiled runtime is. Anyone who considers
that insufficient for their purposes is reading the boundary correctly.

## 3. Platform

The pinned binary is not cross-platform.

| | |
|---|---|
| OS / arch | Linux `aarch64` |
| GPU | NVIDIA GB10-class |
| CUDA | 13 |
| Required libraries | `libcuda`, `libnvrtc`, `libcurand`, `libcublas`, `libcublasLt`, `libcudart` |

`./run doctor` checks all of these and names whichever is missing.

## 4. Decode settings

Passed on the command line by the route. The `[MODEL_SCALE]` line printed at model
load shows a base profile from before these overrides are applied; it is not the
decode configuration and should not be read as one.

```
--temperature 0.0
--max-steps 768
--context-length 8192
--theta-override 1.5
--physics-blend 0.9
--physics-start-layer 16
--physics-end-layer 33
--chat-template auto
--system-prompt-mode free
--output-contract-mode off
--ablate-periodic-controller
--ablate-live-motifs
--workspace-tools false
```

`--physics-end-layer 33` is the argument passed. The architecture has 32 layers,
indices 0–31, so the engine clamps to 31 and the run log reports forces applied
across layers 16–31. Both numbers are correct about their own subject.

## 5. Runtime environment

```
NIODOO_HARD_CLAIM=1
NIODOO_GOD_ZONE_RECOVERY=1
NIODOO_REMEMBER_EAR_PIN_GOAL=1
NIODOO_STRUCTURAL_GOAL=0
NIODOO_REMEMBER_RESIDUAL_EARS=1
NIODOO_DUAL_STREAM=1
NIODOO_DUAL_INJECT_GAIN=1.0
NIODOO_DUAL_POSTURE_BOOST=8
NIODOO_REMEMBER_EAR_MASS=5
NIODOO_REMEMBER_EAR_BETA=1.0
NIODOO_REMEMBER_EAR_LOGIT_BOOST=1.2
NIODOO_REMEMBER_EAR_ORDER_BOOST=0
NIODOO_REMEMBER_EAR_STOP_BOOST=0
```

and these four explicitly unset:

```
NIODOO_REMEMBER_FORCE_ALL
NIODOO_REMEMBER_PROC_ENABLE
NIODOO_REMEMBER_EAR_PROGRESS
NIODOO_STRUCTURAL_GOAL_PHRASE
```

`NIODOO_GOD_ZONE_RECOVERY=1` is load-bearing. Without it the dual stream arms at
roughly `0.01` instead of `4.6` and generation degrades into unusable output. This
is the failure `FLAG_RUN_20260809.md` §6 records as *"without God Zone recovery,
dual force about 0.056; stopped."*

### A note on that variable's name

The mechanism it enables is called **Launchpad** in current documentation. The
environment variable retains the historical name because **it is read by the pinned
binary.** Renaming it would require rebuilding the engine, which produces a different
hash, which is a new coordinate — and would invalidate the byte-identity claim this
entire document exists to support.

The string is therefore frozen for the same reason every other value on this page is
frozen. First mention elsewhere should read *Launchpad (historically "God Zone")*; the
variable is quoted verbatim because that is what the binary reads.

Two arming figures appear in the record and both are correct — they are different
meters:

| measurement point | value |
|---|---|
| arming smoke, step 0 (`SMOKE_GATE.txt`) | `26.137074` |
| during generation (`[DUAL_STREAM] force=`) | `4.617311` |
| unarmed, no recovery | `0.056` documented, `0.0136` measured 2026-08-16 |

Separately: the Launchpad constants stamped 2025-12-16 are **not** the physics settings
in §4 of this document. They belong to a different lane and should not be quoted as the
flag route's configuration. This run's values are `physics_blend=1.50` and
`repulsion_strength=-0.51` as printed at load, with the CLI overrides in §4 applied
after; `[GOD_ZONE] goal_force` ramps 4.5 → 25 across the run.

## 6. Arming gate

The route refuses to proceed unless step 0 reports:

| | |
|---|---|
| `dual_stream_force` | in the historical range 10–40 |
| `dual_stream_n_ear` | 16 |
| `dual_stream_survivors` | 10 |
| ghost basins loaded | 8 |
| residual-ear seed lines | all three present |

At roughly `0.05` the route exits non-zero before the expensive teaching phase
rather than producing a run that looks complete and means nothing.

## 7. The stderr exemption

The recorded two restarts produced byte-identical stdout (20,173,666 bytes),
telemetry (32.2 MB) and score. Their stderr differed by **exactly two bytes**: the
`r1` / `r2` in the output path each process logged for its own telemetry file.

`verify` normalises that one path and nothing else. Any other stderr difference is a
failure. The exemption is hardcoded rather than generalised, so it cannot quietly
grow to cover a real divergence.

## 8. What determinism does and does not establish

The pipeline is deterministic at temperature 0. Two restarts therefore produce
identical bytes by construction.

**This means the second restart is a reproducibility check, not an independent
sample.** Two arrivals establish that the route is repeatable here. They do not
estimate a success rate, and no reading of them should.

Determinism also means **one run per configuration is the complete answer for that
configuration.** That is why `./run sweep` tests a handful of arrangements rather
than repeating each one — repetition would add cost and no information.

## 9. What breaks determinism deliberately

These are available and are **off** in `verify`:

| Feature | Flag | Why it is excluded |
|---|---|---|
| Session resume | `--compact-resume-state-load-file` | Injects prior-session anchors into context |
| KV snapshot restore | `--kv-state-load-file` | Restores transformer state across a restart |
| Per-turn KV reset | `--reset-kv-cache-per-turn` | Changes cumulative context between turns |

`./run chat` uses the first of these by design — a REPL should continue where you
left it. `./run verify` uses none of them.

Raw KV snapshots and the per-turn reset are not surfaced by `./run`. They exist in
the engine, they are large and fragile, and exposing three ways to say "resume"
would obscure the one that a user actually wants.
