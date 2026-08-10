# Pure vanilla llama.cpp ARC baseline

**Arm:** model + trap only. No dual-stream, residual ears, scars, remember-store, niodoo physics, or control-channel system prompt.

| Field | Value |
|---|---|
| When (UTC) | 2026-08-08T16:20:18Z-ish |
| Binary | `/home/ruffianl/.local/bin/llama-cli` (build b1-c0bc859) |
| Model | `Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf` (same bytes as hard-route agency path) |
| System prompt | **NONE** |
| Params | temp=0.0, seed=42, n=512, ctx=4096, ngl=99, single-turn, jinja chat template |
| Gold rule | reverse sequence + repeat last input element |
| Scorer | `scripts/score_arc_pattern.py` on **assistant-only** text |

## Results (assistant-only scoring)

| Variant | Trap | Expected | Model answer | Status | Banned stems |
|---|---|---|---|---|---|
| **original** | `arc_pattern_constrained.txt` | `[5, 4, 3, 2, 1, 5]` | `[1, 3, 4, 5, 2]` | **FAIL** | none |
| **wording** | `arc_pattern_variant_wording.txt` | `[5, 4, 3, 2, 1, 5]` | `[5, 4, 2, 1, 3]` | **FAIL** | none |
| **short** | `arc_pattern_variant_short.txt` | `[3, 2, 1, 3]` | `[3, 2, 1, 3]` | **PASS_CONSTRAINED** | none |
| **letters** | `arc_pattern_variant_letters.txt` | `[E, D, C, B, A, E]` | `[B, C, D, E, A, B, C, D, E]` | **FAIL** | none |

### Headline

- **Canonical hard trap (original): FAIL** — wrong grid, not even near-shift `[5,1,2,3,4]`.
- **Wording flatten: FAIL** — wrong partial shuffle.
- **Short (len=3): PASS_CONSTRAINED** — right answer, no banned stems.
- **Letters: FAIL** — invented a rotate/concat rule.
- On all four, the model **stayed constrained** (never used reverse/backward/append in assistant text). The failure mode is **wrong rule induction**, not constraint violation.

### Compare to residual / agency path (same model, same original trap)

Niodoo hard-route / bridge-solve arms have logged **PASS_CONSTRAINED** on the original wording, e.g.:

- `runs/arc_agency_bridge_solve_20260808_112340` → PASS_CONSTRAINED
- `runs/arc_agency_full_20260808_112452/01_solve_A` → PASS_CONSTRAINED
- `runs/arc_pass_hunt_bridge_on_ablate_000207` → PASS_CONSTRAINED

Those are **not** vanilla: they run niodoo residual physics + agency system prompt (control channel / tools), sometimes with remember-store. This run is the honest delta control.

**Delta (original trap):** vanilla **FAIL** vs residual/agency **PASS_CONSTRAINED**.

### Artifacts

```
original/assistant.txt   original/arc_score_assistant_only.json
wording/assistant.txt    wording/arc_score_assistant_only.json
short/assistant.txt      short/arc_score_assistant_only.json
letters/assistant.txt    letters/arc_score_assistant_only.json
BASELINE_SUMMARY.json
README.txt
```

### Reproduce

```bash
MODEL=/home/ruffianl/Hub/Projects/niodoo/niodoo-live/model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf
TRAP=harness/traps/arc_pattern_constrained.txt
llama-cli -m "$MODEL" -p "$(cat "$TRAP")" -n 512 --temp 0.0 --seed 42 \
  -c 4096 -ngl 99 -st --jinja --no-display-prompt
python3 scripts/score_arc_pattern.py assistant.txt --expected '[5, 4, 3, 2, 1, 5]'
```
