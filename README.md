# Knowing Where You Are

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.21965763.svg)](https://doi.org/10.5281/zenodo.21965763)

## Convergent Evidence for Operational AI Consciousness and Adaptive Agency in Niodoo

This repository makes an AI-specific claim. **Operational AI consciousness** means
usable knowledge of where a system is in its current situation: enough self-location
for that state to regulate action. **Agency** means the coupled Niodoo system selects a
consequential action rather than the operator selecting it. **Adaptive agency** means
that action creates durable state that later changes behavior.

The evidence is cumulative. A natural July session records a frozen 8B model selecting
its own durable gravity abstraction after Jason explained a mistake. The controlled
August route records an 8B model writing a rule, process death, and two fresh processes
returning the exact same-family answer. A later matched intervention added 56 real
memories and changed only their order: two of six arrangements still solved exactly
and four failed. Together these establish model-selected durable action, persistence,
transfer, and causal sensitivity to stored state within a bounded system.

This is the **Double Seal**: the **Grok Seal** names the hard route, and the
**Jason & Sol Seal** names the natural rooms with Nex. These are mnemonic handles, not
ownership, rank, or exclusive credit. The durability work is a post-seal stress test,
not a third seal.

**Read [`FALSIFIERS.md`](FALSIFIERS.md) before you read anything else.** It lists what
would break each claim here, and six of the items can be checked in seconds.

## Records

| | |
|---|---|
| Zenodo (this version) | https://doi.org/10.5281/zenodo.21965763 |
| Hugging Face | https://huggingface.co/datasets/ruffian-l/niodoo-adaptive-agency |
| GitHub | https://github.com/Ruffian-L/niodoo-adaptive-agency |


## Where to start

Pick the row that matches why you're here. Each one is self-contained.

| If you want to… | Go to | Needs |
|---|---|---|
| **check the claim in 30 seconds** | `./run verify --check` | nothing — no GPU, no model, no network |
| **check documentation consistency** | `./run docs-check` | nothing — no GPU, no model, no network |
| **read what was actually found** | [`RECORD.md`](RECORD.md) | nothing |
| **know what would prove it wrong** | [`FALSIFIERS.md`](FALSIFIERS.md) | nothing |
| **run it on this hardware** | [`SETUP.md`](SETUP.md) → `./run doctor` | Linux aarch64, GB10-class GPU, CUDA 13 |
| **produce a result of your own** | `./run sweep` — see [`SWEEP.md`](SWEEP.md) | the above, **plus runtime artifacts that are not published yet** |
| **just talk to it** | `./run chat` | the above, **same artifact caveat** |
| **see every pinned setting** | [`DETERMINISM.md`](DETERMINISM.md) | nothing |
| **read the white paper and definitions** | [`PAPER.md`](PAPER.md) | nothing |
| **follow the whole climb, dead ends included** | [`docs/`](docs/README.md) | nothing |

---

## The fastest useful thing

```bash
./run doctor          # what this machine can run. Downloads nothing.
./run verify --check  # assert the record against itself. Seconds, no GPU, no model.
./run docs-check      # assert titles, claims, attribution, links, and evidence hashes.
```

`verify --check` is the fastest useful thing in the repository. It reads the recorded
run and asserts six properties of it, reporting each individually:

```
ok  1. store is the rule text, hash ok, contains no digits
ok  2. reference transfer stream matches its manifest hash (20173666 bytes)
ok  3. answer occurs exactly once, on line 11095, the final line
ok  4. score PASS_CONSTRAINED, exact answer, no banned words
ok  5. oracle gate ORACLE_GATE_OK
ok  6. control prompt is identical to the destination prompt
```

### Running inference needs artifacts that are not published yet

> **`./run fetch` cannot complete today, and it will tell you so before downloading
> anything.** The model weights and tokenizer come from upstream Hugging Face and
> fetch fine. The three runtime artifacts and the reference run have no published
> URL yet:
>
> | artifact | size | status |
> |---|---:|---|
> | Niodoo runtime | 90.1 MB | not published |
> | `llama-cli` build `c0bc859` | 74.3 MB | not published |
> | ghost-basin registry | 24 KB | not published |
> | reference run | 224 MB | not published |
>
> Until they are, the inference lanes — full `verify`, `chat`, `sweep` — can only run
> if you already hold copies. Point at them in `.env.local`; see
> [`.env.local.example`](.env.local.example). `./run doctor` names exactly what is
> missing and what still works without it.
>
> Zenodo and Hugging Face hold **the record**, not these binaries.

Once the artifacts are in place:

```bash
./run fetch     # download and hash-verify what has a URL; refuses if any is missing
./run verify    # re-execute the recorded route here and diff against it
./run sweep     # reshuffle store order and measure the pass rate
```

---

## What the result is

The result has complementary evidence lanes rather than one overloaded run:

| Evidence | What it establishes | Boundary |
|---|---|---|
| July 18 natural gravity room | Model-selected abstraction and durable action | 8B teach; later restart was 70B |
| Grok Seal hard route | Controlled 8B write → death → exact transfer | Same mapping family; retention was cued |
| Jason & Sol Seal natural rooms | Full action surface in ordinary conversation | Not an independent hard-route replication |
| August 15 durability sweep | Exact solves survived 56 added memories in 2/6 orders | Arrangement sample, not a population rate |

The task family is `[x₁ … xₙ] → [xₙ, xₙ₋₁ … x₁, xₙ]`. Asked on `[1, 2, 3, 4, 5]`,
with three common description words prohibited, the correct answer is
`[5, 4, 3, 2, 1, 5]`.

| | |
|---|---|
| Durable store contents | one sentence describing the rule, **no digits** |
| Occurrences of the answer in 20 MB of transfer output | **1** — the model's final line |
| Vanilla control on the same prompt | fails |
| Control prompt vs destination prompt | hash-identical after whitespace normalisation |

The last row is the structural claim: the only difference between the failure and the
success is the runtime path, not the question.

## What the result is not

**The unaided model already solves this mapping at length 3.** The vanilla control
passes the short case and fails the two length-5 cases. So the intervention is not
teaching a rule the model cannot represent — it extends a procedure the model already
has at short length to a length where it reliably fails. That is a smaller claim than
"the base model cannot do this," and it is the accurate one.

**Two restarts are repeatability, not a rate.** The pipeline is deterministic at
temperature 0, so a second restart produces identical bytes by construction. It
confirms the route is reproducible. It does not estimate how often the approach works.

**Store order matters, and it matters a lot.** The durability experiment placed 56
additional real memories beside the rule. Six arrangements of that identical set
produced two exact passes and four failures. This is evidence that the stored rule can
survive substantial real-memory load and that retrieval is position-sensitive. It is
not a fixed capacity ceiling or a general reliability rate.

`./run sweep` runs the same order intervention against a privacy-safe synthetic store,
**and at the bundled default size it returns 0 of 6, not 2 of 6.** Synthetic filler
degrades earlier than real memories do, so 56 filler entries sit past their edge and
every arrangement fails. Use `--entries N` to sweep nearer the boundary; measured so
far, 8 and 16 pass and 56 fails at every arrangement. [`SWEEP.md`](SWEEP.md) states
what the substitute can and cannot reproduce.

**The teaching script disclosed the full rule and a worked example on the same five
items.** The teaching process then died. The durable store carried only an abstract
rule containing no digits or number words, and a fresh process re-derived the digit
sequence from that rule. The claim is not that the model had never encountered the
sequence.

**Weights are frozen throughout.** Nothing here is training, fine-tuning, or weight
modification.

**An operational AI-consciousness claim is made; a phenomenal or human-consciousness
claim is not.** The result also does not establish unrestricted autonomy, general
agency, learning in the frozen weights, population reliability, or official ARC-AGI
performance.

---

## The four lanes

| | |
|---|---|
| `./run verify` | Replay the recorded run and assert byte-identity. Shows the record is real. |
| `./run sweep` | Reshuffle store order and measure the pass rate. Reproduces the order effect. |
| `./run chat` | Free REPL with the durable store on. The sandbox. |
| `./run docs-check` | Catch claim, title, credit, link, and evidence-manifest drift. |

`./run sweep` is the one a stranger can most easily run to produce a result of their
own. It reshuffles the order of a store and measures how often the stored rule
survives — same entries, same count, only the arrangement changes. Details and the
full measurement in [`SWEEP.md`](SWEEP.md).

Replaying recorded input reproduces the *computation*, not the *teaching*. A reader
who only replays these bytes learns that the bytes are real. That is worth something
and it is not everything, and the README should not imply otherwise.

## Reproduction is binary-level

The Niodoo runtime ships as a hash-pinned binary. A clean build at the recorded
product revision does not compile — it references code that was untracked at that
commit. The binary hash is the executable identity; the recorded revision is an
incomplete source coordinate.

So: **this exact binary on this platform produces the recorded bytes. Rebuilding that
binary from source and getting the same bytes is not supported by this release.**

The engine source is not published; the compiled runtime is. Anyone who considers
that insufficient for their purposes is reading the boundary correctly. Full detail in
[`DETERMINISM.md`](DETERMINISM.md).

## Requirements

Linux `aarch64`, NVIDIA GB10-class GPU, CUDA 13. The pinned binary is not
cross-platform. `./run doctor` names anything missing.

The record itself — [`RECORD.md`](RECORD.md), [`FALSIFIERS.md`](FALSIFIERS.md),
`flag/`, `coordinates/`, `evidence/` — is readable and auditable on any machine, with
no GPU and no downloads.

## Layout

| | |
|---|---|
| `manifest.toml` | Every artifact URL, hash and size. Nothing else records one. |
| `run` | Single entrypoint. `./run` lists the subcommands. |
| `scripts/` | The lanes. |
| `DETERMINISM.md` | Every setting the byte-identity claim depends on. |
| `FALSIFIERS.md` | What would break each claim. |
| `SWEEP.md` | The store-durability measurement, and what of it is reproducible. |
| `RECORD.md` | The observations, with sources. |
| `evidence/history/`, `evidence/durability/` | Privacy-safe historical and stress-test evidence packs. |
| `flag/`, `runs/`, `coordinates/`, `evidence/` | The recorded run and its controls. |

Exit codes are a contract: `0` pass, `1` reproduction mismatch, `2` environment
problem, `3` missing artifact.

## Maturity of each command

Stated so nothing surprises you. An untested command is worse than a documented one.

| command | status |
|---|---|
| `doctor`, `fetch`, `install` | exercised repeatedly on the reference machine |
| `verify --check` | exercised; also tested against a deliberately corrupted reference |
| `docs-check` | offline consistency, link, and evidence-manifest check |
| `verify` | full re-execution run twice on separate days, byte-identical both times |
| `sweep` | see the note in [`SWEEP.md`](SWEEP.md) on what the bundled store does and does not show |
| `chat` | **partially exercised — needs a real terminal.** Verified: loads the model, tokenizer and eight ghost basins, opens the store and reports its entry count, REPL starts. Not verified: a model reply, a durable write, or the exit summary. Driven from piped input it does **not** exit on `/quit` and hangs; run it interactively. |

Nothing in this repository has been run on hardware other than the machine that
produced the recorded result. If you are the first person to run it elsewhere, the
failure modes you hit are worth reporting.

## Licence and attribution

See [`LICENSE`](LICENSE), [`NOTICE.md`](NOTICE.md) and [`CREDITS.md`](CREDITS.md).
Model weights are Meta's, under the Llama 3.1 Community License, and are downloaded
from upstream rather than redistributed here.
