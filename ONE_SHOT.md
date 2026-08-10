# One-Shot Route

**Historical route status: FLAG PLANTED. Matched-control status: wired and byte-verified, pending a new coordinate.** The complete 2026-08-09 route passed: control, force `26.137074`, beta `1.0`, empty-store teaching, autonomous two-entry save, process death, `PASS_CONSTRAINED ×2`, and oracle gate OK ×2. Its control phase used no system prompt. The active runner now emits and saves complete raw subprocess streams while giving `llama.cpp` the same free-mode system message as Niodoo; that changed control must be recorded as a new run.

Run the full route with:

```bash
cargo one-shot
```

The order is fixed:

1. Control: four `llama.cpp` conversations with the byte-matched Niodoo free-mode system prompt, but no Niodoo physics, store, residual ears, or dual stream.
2. Mandatory smoke: one Niodoo token with the historical scar store and exact flag environment.
3. Stop unless step 0 has FLAG-like force, 16 ears, about 10 survivors, and all three residual-ear seed lines.
4. Only after arming is restored: Niodoo physics and human teaching from an empty store.
5. Self-save, death, and two fresh wording-only restarts.
6. A flag is planted only when the authoritative scorer reports `PASS_CONSTRAINED` twice and the authoritative oracle gate reports OK twice.

## Full conversation rule

There is no assistant-output parser in the active one-shot runner. Child stdout and stderr are teed byte-for-byte to the terminal and to files. The complete `llama.cpp` generated-conversation file is also printed byte-for-byte. Nothing is shortened into a preview or assistant slice.

The terminal host may limit how much scrollback the editor displays. The files under `runs/one-shot-*` are the complete streams and are not truncated by the runner.

## Matched system-prompt control

The live runner passes [route/system_prompt.txt](route/system_prompt.txt) to `llama-cli` as an explicit system message after removing only the file's final newline. This is the same free-mode control-channel text embedded in the pinned Niodoo binary. Workspace tools are disabled on the transfer lane, so neither side receives a tools block. Each live control directory records the exact bytes as `system-prompt.txt`.

The archived 2026-08-08 vanilla coordinates predate this change and intentionally remain labeled `System prompt: NONE`. They are not rewritten. A matched-prompt execution is a new coordinate and must retain its own outputs.

The only field reads before a full run are the three step-0 telemetry numbers used by the arming gate. Model conversation text is not parsed or scored at smoke time.

The smoke gate requires first-token `dual_stream_force` in the historical order of magnitude (`10..40`), `dual_stream_n_ear=16`, `dual_stream_survivors=10`, and the residual-only, procedure-clause, and `ORDER=0` seed lines. At approximately `0.05`, the runner exits nonzero before the expensive teaching route.

## Path overrides

The defaults point to the original machine. Override any moved component with:

| Variable | Component |
|---|---|
| `ONE_SHOT_LLAMA_CLI` | `llama-cli` binary |
| `ONE_SHOT_NIODOO_BIN` | pinned Niodoo binary |
| `ONE_SHOT_MODEL` | pinned GGUF |
| `ONE_SHOT_TOKENIZER` | tokenizer JSON |
| `ONE_SHOT_SOURCE_ROOT` | `niodoo-arc-rehit` working root |
| `ONE_SHOT_PRODUCT_ROOT` | product root containing bridge assets |
| `ONE_SHOT_OUT` | output directory |

Model and Niodoo binary hashes are mandatory. A different binary is a new coordinate and must not be presented as this byte-identical route.

See [SETUP.md](SETUP.md) for the portable map-check lane, exact CUDA lane, automatic model/tokenizer downloads, and the binary-release milestone that remains open.

## Separate full-tools preflight

The historical `cargo one-shot` lane remains pinned and keeps workspace tools off. The merged organic-route plumbing uses a different fail-closed command and a different binary hash:

```bash
cargo merged-preflight
```

It obtains the complete full-tools prompt directly from Niodoo, gives those exact bytes to llama.cpp, then checks an isolated live `write/read` loop with Qdrant off. It does not script the organic gravity conversation, run the five-phase flag route, or claim an earned transfer. See [MERGED_PREFLIGHT.md](MERGED_PREFLIGHT.md).
