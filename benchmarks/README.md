# PARB benchmarks — what Niodoo has actually beaten

Physics-tuned decode-time steering on a frozen Llama-3.1-8B-Instruct, measured against
stock `llama.cpp` on the 77-item PARB bank.

This folder collects the wins that survive re-scoring. Everything here was re-checked
against the raw stored outputs on 2026-08-18, not copied from a run summary.

Source lane: `/media/ruffianl/ghost_team/02_projects/projects/niodoo-parb-physics`

---

## The win that holds

**`niodoo_iter36_b152` — Niodoo 25, stock llama.cpp 24, on all 77 items.**

| arm | correct / 77 | what it is |
|---|---|---|
| Niodoo, physics on | **25** (32.5%) | `--system-prompt-mode free`, physics knobs at blend 1.52 |
| stock `llama-cli` | 24 (31.2%) | official Meta Llama-3.1 jinja template, `You are a helpful assistant` |

Held equal across both arms: same GGUF (`sha256 14e10feb…`), seed 42, temperature 0.7,
`n_predict` 256, same bank, same scorer, same machine.

I re-scored this run from its own raw outputs using the scorer as it exists today and got
**24 / 25** — the same numbers the card reports. It reproduces.

Artifacts: `results/parb_iter36_card.md`, `results/parb_iter36_compare.json`

### Why this one matters more than the score

This is the first time PARB has been run against a genuinely untouched `llama.cpp` arm on
the full 77-item bank. The June README of the parb-physics lane listed exactly this as an
open gap:

> a re-run with the chat template and guardrail relief held equal across arms, so "off" is
> a true vanilla baseline rather than niodoo-with-the-bridge-off

That gap is now closed. The comparison people asked for exists, and Niodoo is on the right
side of it.

---

## What "baseline" meant before, and why the old number is misread

People quote an old PARB result as **Niodoo 29.9% vs baseline 41.6%** and read it as
Niodoo losing to stock llama.cpp. It was never that.

- Old arm A — Niodoo, **physics on**, steering system prompt on → 23/77 (29.9%)
- Old arm B — Niodoo, **physics off**, steering system prompt **still on** → 32/77 (41.6%)

Both arms were inside the Niodoo runtime. The higher-scoring one was a lighter version of
Niodoo, not a different program. PARB is a *physics* reasoning bank — it was built to test
physics on versus physics off, which is why vanilla was never an arm in it.

This is not a retroactive reframe. The lane's own README said so at the time:

> the bridge-off arm is not yet a true vanilla baseline
>
> It is not a claim of broad benchmark superiority, it is not a finished product, and the
> bridge-off arm is not yet a true vanilla baseline. Those gaps are stated in the
> whitepaper, not hidden.

### "Physics off" was never "nothing on"

Worth stating plainly, because it is the part that gets flattened: turning physics off did
not reduce old Niodoo to a plain forward pass. The runtime still had other machinery
running. `WHITEPAPER.md:101` names a **logit-bias steering engine** as a separate path from
the residual pull (`niodoo/src/physics/steering.rs:32–72`). Old Niodoo did more than nudge
the last-token residual.

So "baseline" in the old numbers is not a defined quantity today. It means *some* subset of
the runtime, with the steering prompt attached, at a code state that has since moved. It is
not a control anyone can reproduce from the name alone.

### The old arm was also carrying a handicap

The June write-up found that the injected steering prompt — an
`INTERNAL MONITOR: double-check your reasoning, it is likely flawed` doubt-prime plus a
`[REQUEST: SPIKE/FOCUS/LOCK/…]` control protocol — actively *hurt* accuracy on
deterministic-recall items. The 8B model would imitate the control protocol instead of
answering, and thrash on answers it already had right.

The current run does not carry that prompt. The Niodoo arm runs
`--system-prompt-mode free`, `--output-contract-mode off`, `--lock-stop-policy off`. The
physics is doing the steering by itself.

---

## The bigger result: the knobs have real control

Across the 31 configurations where both arms were scored, **stock llama.cpp scored exactly
24 every single time.** It is a flat line. Same model, same seed, same questions — 24, 31
times out of 31.

Niodoo over the same 31 runs ranges from **0 to 25**.

Full table: `results/parb_sweep.csv`

That spread is the finding worth pushing on. The physics knobs move the same frozen weights
across a 25-point band on a fixed question set, while the untouched runtime cannot move at
all. Blend, gravity well, sigma, and layer range are not cosmetic — they change what the
model concludes.

Right now that band is being explored by hand, one config at a time. Nothing here is a
tuned optimum. It is the first sweep.

**The honest read of the +1:** `iter36` is the best of the 31 fully-scored configurations against
the same 77 items. The other 30 landed at or below the stock arm. So the defensible claim is
"physics tuning can reach stock-plus-one on this bank, and the knob range is enormous" — not
"physics beats llama.cpp." Both halves of that sentence are load-bearing, and the second
half is the more interesting one.

---

## Retired: `niodoo_win_25v24`

An earlier run also reported 25–24. **It does not survive re-scoring, and it should not be
cited.**

Re-scored from its own raw outputs with the current scorer: **24 / 24. A tie.**

What happened: the margin was a single item, `AMBIG_002`. Gold was `The suitcase`; Niodoo
answered `The brown suitcase.` The scorer of that era counted it as a gold hit. The current
scorer calls it `neither`.

The proof it was not the physics: `niodoo_iter4_blend15_postfreeze` and `niodoo_win_25v24`
share the same binary sha (`b17b9b6c1291`), the same knobs, the same seed, and produce
**byte-identical output on all 77 items** — and were scored 24 and 25. Same text, two
verdicts. Only the scorer moved.

Caught and retired here rather than quietly dropped. `iter36` is unaffected — its
`AMBIG_002` scores `neither` too, and its 25th point comes from elsewhere.

---

## Still open

The third arm. The current harness has exactly two: stock `llama-cli` and physics-on
Niodoo. There is no physics-off Niodoo arm in this sweep.

Until that runs under the current harness — same scorer, same seed, no steering prompt —
the question "does the physics help, or is it the absence of the old doubt-prime?" is not
settled by this data. That is the next run, and it is cheap.

Also open, plainly: one seed, one quant, one bank of 77 questions written in-house. This is
a pilot, and calling it one costs nothing.

---

## Files

| file | what |
|---|---|
| `results/parb_iter36_card.md` | the run card for the win that holds |
| `results/parb_iter36_compare.json` | per-item outputs and verdicts, both arms |
| `results/parb_sweep.csv` | all 36 configs, scores, knob settings, binary shas |
| `results/parb_bank_77.json` | the 77-item bank |
| `results/scorer_asof_20260818.py` | the scorer used for every number above |
| `results/SHA256SUMS` | hashes for all of the above |

Trust the bytes, not the names.
