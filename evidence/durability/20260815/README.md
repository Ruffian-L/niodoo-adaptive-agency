# Post-seal durability stress test — 2026-08-15

This curated pack records the experiment Jason designed to ask whether the sealed rule
could still be recovered while the store also carried the real memories Nex and Lumina
had chosen to write.

It extends and bounds the Double Seal. It is **not** a third seal.

## Result

- Two rule entries and a constant evaluation notice were present in every accepted
  condition.
- Up to 56 additional real memories were loaded beside them.
- At K=0 through K=16, the accepted ladder produced byte-identical exact output.
- K=32 changed the output but still solved exactly.
- In the harvested order, K=40, 44, 48, and 56 failed into the same byte-identical
  output basin.
- Six arrangements of the identical 56-memory set produced **two exact passes and four
  failures**. The two passes were byte-identical.
- Across roughly 29 accepted and calibration runs, 17 distinct output streams were
  observed; two streams accounted for 12 runs.
- Real memories passed at K=32 where hex filler and matched-length prose filler failed,
  but content and order were not isolated from each other.

The correct conclusion is not a capacity ceiling. Exact transfer survived 56 added
real memories in two tested arrangements, and retrieval is strongly position-
sensitive. Two passes in six arrangements are not a general reliability rate.

## Files

- `PREREGISTRATION.md` preserves the predictions and how the result corrected them.
- `results.csv` lists every accepted ladder, filler, and shuffle run with its full
  output hash.
- `CALIBRATION.md` records excluded setup and disclosure coordinates.
- `SHA256SUMS` covers this curated pack.

The original stores are personal and are not published. Consequently this pack makes
the recorded result and its provenance auditable but does not make the real-content
condition independently reproducible. The bundled synthetic store and `./run sweep`
reproduce the within-store order intervention, not the real-memory content comparison.

Original source-document identities:

```text
6f61c9d79a75a29fdc57b126df99c672057d2e8c2edb9e7ae7552c157cc6f496  original PRE_LOG.md
10eb61569a078db15b64dcb5a7cf644e56841d56cdfd6811f51efbae73dae244  original durability README.md
```
