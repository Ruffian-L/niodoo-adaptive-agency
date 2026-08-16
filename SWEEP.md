# Store durability: order, not volume

*This page measures a limitation of the retrieval path. It does not bear on whether the
recorded result happened: that route reproduced from cold on 2026-08-15 and again on
2026-08-16, byte-identical both times. See [`PAPER.md`](PAPER.md) for the result and
[`docs/seal/SEAL_20260814.md`](docs/seal/SEAL_20260814.md) for what was sealed.*

Measured 2026-08-15. Roughly 29 runs, each holding the task and every runtime setting
at the sealed configuration and varying only `--remember-store`.

**No store contents are published.** The store used in the original measurement is
personal and is not in this repository. Results below are counts. What ships is a
synthetic store of matched length plus the harness, which is sufficient to reproduce
the order effect and is not sufficient to reproduce the content effect — see the
named limitation at the end.

---

## 1. The K ladder — volume alone does not explain it

Two rule entries, the constant evaluation notice, and K additional entries in one
fixed order:

| K | result | output |
|---:|---|---|
| 0, 1, 2, 4, 8, 16 | exact | **byte-identical across all six** |
| 32 | exact | shifted, answer still correct |
| 40, 44, 48, 56 | wrong | degraded |

Up to 16 additional entries the output does not change by a single byte. The entries
are present and have no effect on generation at all. Between 16 and 32 the output
begins to shift while remaining correct. From 40 it degrades.

The engine reports `total_ear_keys=16`, and 16 is where any effect at all begins.

## 2. The order effect — the load-bearing result

The same 56 additional real memories, same count, same content, with the rule entries
held at fixed positions. Only the permutation of those 56 memories changes.

| seed | result | decode tokens | output hash |
|---:|---|---:|---|
| 1729 | **PASS** | 361 | `8aa2de66…` |
| 11 | FAIL | 469 | `2ceca8d0…` |
| 227 | FAIL | 469 | `ccd5995b…` |
| 3313 | FAIL | 469 | `ccd5995b…` |
| 44497 | FAIL | 375 | `c9949440…` |
| 501013 | **PASS** | 361 | `8aa2de66…` |

**A set that fails in one arrangement passes in another, and the two passing streams
are byte-identical.** The ladder in §1 therefore describes an arrangement, not a
capacity. Any single number for "how many entries fit" is a property of one
permutation. Two passes in six arrangements are not a general reliability estimate.

Reproduce on your own machine:

```bash
./run sweep --shuffles 6
```

## 3. Collapse into a small number of states

Across all runs:

| | |
|---|---|
| runs | ~29 |
| distinct output streams | **17** |
| runs accounted for by the two most common streams | **12** |

Two unrelated shuffle seeds produced byte-identical passing output. Four different
store sizes in the fixed order produced byte-identical failing output.

Generation is not degrading smoothly under load. It falls into a small set of discrete
states, and different stores land in the same one.

## 4. Failure signature

The stored rule is *start at the end*. A failing run **begins from the start of the
list** — the default reading direction an untaught model uses — and then reasons
coherently, enumerating each position, to a wrong answer.

Not noise, and not confusion: a well-formed derivation from a first step the store did
not supply.

```
K=32, correct:  "the first element in the output list is the second element
                 from the right in the input list"

K=40, failing:  "In Pair A, the first element is not moved. The second element
                 is moved to the third position."
```

## 5. What this implies for the retrieval path

A picker whose result depends on where an entry sits, and which collapses into a
handful of attractor states, does not have a capacity problem. Adding room would not
address it. The variable to remove is position.

---

## Named limitation: the content result is not reproducible from this repository

One measured result is **deliberately not shipped in reproducible form.**

At K=32, entries from the original store produced a correct answer where two
independent synthetic filler sets — one hex-based, one plain English of matched
length — both failed. That is a content difference in the recorded conditions, but
not an isolated content effect because order was not matched across repeated trials.

It is not reproducible here, because reproducing it would require publishing the
original store, and the store is personal.

**The synthetic store in `reference/sweep-store-synthetic.jsonl` is not a substitute
for that condition and must not be presented as one.** It can reproduce the
within-store permutation design in §2: the same set is held fixed and only permuted.
It cannot reproduce the real-memory condition or settle the content comparison.

Stated as a hole rather than approximated, on the same basis as the source-build limit
in `DETERMINISM.md` §2.

Two further constraints on how far §content should be read, both of which apply
against it:

- the real-entry condition at K=32 is a **single arrangement**, and §2 shows
  arrangement alone can flip a result;
- content and order are therefore **not separated** by this measurement.

The honest summary is: exact transfer survived 56 added real memories in two tested
arrangements; order is established as causal, content is suggested and unresolved,
and no fixed capacity ceiling was found.
