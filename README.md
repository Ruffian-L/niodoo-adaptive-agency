# Adaptive Agency in a Frozen 8B Model

**Loop-breaker:** teach scar -> process death -> exact transfer on a different wording of the same ARC-style family.

**Double seal (2026-08-14):** two seals, one MVP. First seal is the planted hard-route flag (Grok, with Jason). Second seal is Jason confirming the same MVP in the natural agency rooms so the first cannot fall out. See [SEAL_20260814.md](SEAL_20260814.md).

**Current one-shot status (2026-08-09): FLAG PLANTED.** The parser-free runner preserved every model byte, restored arming at force `26.137074`, beta `1.0`, 16 ears, and 10 survivors, recreated the two-entry scar from an empty store, ended the teaching process, and started two fresh wording-only processes. Both scored `PASS_CONSTRAINED`; both oracle gates returned OK.

**Control-surface note:** that archived full run used the earlier no-system `llama.cpp` control. The live runner now supplies `llama.cpp` with the byte-identical Niodoo free-mode system message and records it beside every control transcript. This wiring is byte-verified against the captured Niodoo input; its next execution is a new coordinate, not a retroactive change to the flag run.

**Full-tools preflight (2026-08-09): PASS.** A separate new-coordinate binary now exposes the exact resolved full-tools prompt for matched control, confines tools and Remember writes to per-run homes, rejects path escapes, and runs with Qdrant off. The matched llama control still missed the correct answer. In Niodoo's mechanical smoke, the model-authored output emitted `write`, `read`, `<remember>`, and `<lock>`; the engine receipts confirm execution and durable acceptance. This is plumbing evidence, not the claimed organic merged run. See [MERGED_PREFLIGHT.md](MERGED_PREFLIGHT.md) and the untouched raw streams it links.

This repository is the public map of one reached destination. A frozen Llama 3.1 8B Instruct model wrote a general mapping rule to a durable store during an earlier teaching session. After the process ended, two independent restarts loaded that store and produced the exact answer on a wording-changed task from the same family.

We call this **adaptive agency** under that operational definition. General agency, weight learning, official ARC-AGI performance, and unrelated-family transfer remain unmarked territory.

## Result

Correct answer: `[5, 4, 3, 2, 1, 5]`

| Arm | Attempt | Actual | Result |
|---|---|---|---|
| Niodoo, frozen store | Restart 1 | `[5, 4, 3, 2, 1, 5]` | `PASS_CONSTRAINED` |
| Niodoo, frozen store | Restart 2 | `[5, 4, 3, 2, 1, 5]` | `PASS_CONSTRAINED` |
| Vanilla `llama.cpp` | Original wording | `[1, 3, 4, 5, 2]` | `FAIL` |
| Vanilla `llama.cpp` | Changed wording | `[5, 4, 2, 1, 3]` | `FAIL` |
| Vanilla `llama.cpp` | Short case | `[3, 2, 1, 3]` | `PASS_CONSTRAINED` |
| Vanilla `llama.cpp` | Letter tokens | `[B, C, D, E, A, B, C, D, E]` | `FAIL` |

The vanilla short-case pass matters. The control is not presented as incapable of the rule; it failed the canonical, changed-wording, and letter-token cases under the pinned settings.

## Check it

The map checker is Rust with no third-party dependencies:

```bash
cargo run --locked
```

It checks both restart outputs, their scores and oracle gates, prompt/store separation, flag settings, all four vanilla coordinates, and the vanilla run configuration.

## Run the whole route

On the original machine, one command first runs `llama.cpp` with the same free-mode system prompt used by Niodoo, then executes the complete teaching/death/transfer route:

```bash
cargo one-shot
```

The command verifies the pinned model and Niodoo binary and writes every system prompt, user prompt, and transcript under `runs/one-shot-*`. The matched control has no Niodoo physics, store, residual ears, or dual stream. Machine-specific path overrides are documented in [ONE_SHOT.md](ONE_SHOT.md).

To verify the separate full-tools organic-route plumbing before a user-operated natural session:

```bash
cargo merged-preflight
```

That command is intentionally not an alias for the historical flag route and cannot plant a flag.

Start with [LANGUAGE.md](LANGUAGE.md), [FLAG_CARD.md](FLAG_CARD.md), [CLIMB_CARD.md](CLIMB_CARD.md), and [PAPER.md](PAPER.md). The reached destination is under [flag/](flag/); comparison terrain is under [coordinates/](coordinates/). [CLIMB_MAP.md](CLIMB_MAP.md) shows the route and [MILESTONES.md](MILESTONES.md) records the boosts that refueled it.

## Map edge

This repository contains the map and an offline checker. It does not vendor the Niodoo engine, `llama.cpp`, model weights, tokenizer, or binaries. Their identities are pinned in [TRUST_THE_BYTES.md](TRUST_THE_BYTES.md). See [REPRODUCE.md](REPRODUCE.md) for the distinction between checking this map and generating a fresh coordinate.
