# Climb Card: Wording Transfer After Restart

> **Historical climb note (2026-08-16):** This card describes the Grok Seal route at
> the time it was mapped. See [`PAPER.md`](../../PAPER.md) and
> [`RECORD.md`](../../RECORD.md) for the later Double Seal, July evidence, and
> durability measurement.

**Who and when:** Human lead Jason Van Pham; original run 2026-08-08; map assembled 2026-08-09 with GitHub Copilot. Source tree `niodoo-arc-rehit@8033ec2`.  
**Destination:** REACHED

## 1. Starting question

Would a frozen 8B model use a general rule scar written during teaching, survive process death, and produce the exact constrained answer on a differently worded task from the same mapping family?

## 2. Terrain crossed

- Model: Llama 3.1 8B Instruct Q5_K_M.
- Store: the frozen two-entry rule scar in `flag/store_at_flag.jsonl`.
- Destination: the single-turn prompt in `flag/session.txt`.
- Attempts: two separate deterministic process restarts, `r1` and `r2`.
- Comparison coordinates: four vanilla `llama-cli` variants, temperature 0, seed 42, no system prompt.

## 3. Route settings

The map preserves the settings, prompt, model and binary hashes, outputs, and gates. The literal original shell invocation was not retained, so this card does not invent one.

| Setting | Value |
|---|---|
| Runtime scaling profile | `7b` |
| Residual ears / dual stream | on / on |
| Mass / beta / inject / posture / logit | `5 / 1.0 / 1.0 / 8 / 1.2` |
| Order / stop / progress digit tip | `0 / 0 / off` |
| Temperature / maximum steps | `0.0 / 768` |
| Physics layers / blend / theta | `16-33 / 0.9 / 1.5` |

The public map is checked with:

```bash
cargo run --locked
```

## 4. Destination written on the map

Exact list `[5, 4, 3, 2, 1, 5]`, with none of the forbidden explanation words. The lean batch does not independently establish when that destination was first written down.

## 5. Arrival

Restart 1 returned `[5, 4, 3, 2, 1, 5]`; correct answer `[5, 4, 3, 2, 1, 5]`; `PASS_CONSTRAINED`; oracle gate OK.

Restart 2 returned `[5, 4, 3, 2, 1, 5]`; correct answer `[5, 4, 3, 2, 1, 5]`; `PASS_CONSTRAINED`; oracle gate OK.

Vanilla returned the wrong answer on the original and changed-wording length-five tasks, reached the short-task destination, and took a wrong route on the letter-token task. Every correct answer is printed in [FLAG_CARD.md](../seal/FLAG_CARD.md).

## 6. Coordinates and milestones

| Kind | Hypothesis or boost | Coordinate |
|---|---|---|
| Milestone | Pure residual, inject 1.5 / posture 12 | Near destination `[5, 4, 3, 2, 2, 5]` |
| Coordinate | Multi-scar / triple-mass clauses | Unstable output |
| Coordinate | Procedure clauses off | `[2, 3, 1, 4, 5]` |
| Boundary marker | Progress digit tip | Disallowed route; turned away |
| Milestone | Pure residual, inject 1.0 / posture 8 | Refueled the route; flag held twice |

## 7. The math, in plain words

Both recorded deterministic restarts arrived. Their byte identity establishes
repeatability of one trajectory, not statistical independence or a reliability rate.

Vanilla reached one of four variant destinations: `1 / 4 = 25%`. It reached zero of one changed-wording length-five destinations. Four heterogeneous prompts are too few to map benchmark-wide behavior.

## 8. Milestone decision

The stronger dual settings stabilized much of the end-to-start walk but duplicated the penultimate item. The decision was to reduce inject gain from 1.5 to 1.0 and posture boost from 12 to 8 so the stored rule geometry could complete the sequence without direct digit guidance. Paths that injected rule text or installed gold-token order were declared void.

## 9. Human map check

- [x] Recomputed `cargo run --locked` (2026-08-09).
- [ ] Compared model, tokenizer, and binary hashes.
- [ ] Read both assistant transcripts and all four vanilla transcripts.
- Initials/date: ____________________
