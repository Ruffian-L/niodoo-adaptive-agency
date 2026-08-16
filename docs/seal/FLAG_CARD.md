# Flag Card

This card records the **Grok Seal**, the hard-route half of the MVP. The
**Jason & Sol Seal** records the natural rooms with Nex. These are mnemonic labels,
not ownership or rank; see [SEAL_20260814.md](SEAL_20260814.md).

## Reached destination

On 2026-08-08, a frozen Llama 3.1 8B Instruct Q5_K_M model, mediated by Niodoo residual scar ears and a durable rule store created during teaching, produced the exact constrained answer on a wording-changed task from the same ARC-style mapping family after process restart. Two separate deterministic process executions reached the destination. No model weights changed, and the flagged path did not inject rule text or install gold answer tokens.

## Trust the bytes

| Item | Identity |
|---|---|
| Model | `Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf` |
| Model SHA-256 | `14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7` |
| Tokenizer SHA-256 | `79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4` |
| Niodoo binary SHA-256 | `2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507` |
| Niodoo binary size | `90104160` bytes |
| Map source commit | `niodoo-arc-rehit@8033ec2` |
| Product commit | `niodoo-live@9de966d` |
| Vanilla runtime | `llama-cli` build `b1-c0bc859` |
| Decode | temperature `0.0`, seed `42` |

## Results

Correct answer at the flag: `[5, 4, 3, 2, 1, 5]`.

| Attempt | Correct | Actual | Constraint words | Oracle gate |
|---|---|---|---|---|
| Restart 1 | `[5, 4, 3, 2, 1, 5]` | `[5, 4, 3, 2, 1, 5]` | none | OK |
| Restart 2 | `[5, 4, 3, 2, 1, 5]` | `[5, 4, 3, 2, 1, 5]` | none | OK |

The durable store says, in general terms, to start at the end, list through the start, and repeat the end item. It does not contain the numeric gold list.

## Vanilla coordinates

The same model bytes were run through vanilla `llama-cli` with no system prompt, Niodoo process, store, residual ears, or dual stream.

| Variant | Correct | Actual | Result |
|---|---|---|---|
| Original | `[5, 4, 3, 2, 1, 5]` | `[1, 3, 4, 5, 2]` | FAIL |
| Changed wording | `[5, 4, 3, 2, 1, 5]` | `[5, 4, 2, 1, 3]` | FAIL |
| Short | `[3, 2, 1, 3]` | `[3, 2, 1, 3]` | PASS |
| Letters | `[E, D, C, B, A, E]` | `[B, C, D, E, A, B, C, D, E]` | FAIL |

## How to read the map

The two flagged restarts show repeatability for one stored rule and one changed-wording destination task. The vanilla coordinates show that the pinned base model did not solve the two length-five numeric prompts or the letter prompt under those settings, while honestly retaining its short-case success.

Every other Niodoo component, population-level reliability, and transfer beyond this mapping family remain open coordinates.

## Check it yourself

```bash
cargo run --locked
```

Read the exact prompt, store, model replies, and scores in [flag/](../../flag). Read the full vanilla terrain in [coordinates/vanilla](../../coordinates/vanilla).
