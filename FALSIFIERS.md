# Falsifiers

Each item below would break a specific claim this project makes. They are listed
first-class rather than buried, because a claim you cannot break is not a result.

Where a falsifier can be checked mechanically, the command is given. Several can be
run in seconds on any machine, with no GPU and no model.

---

## Checkable now, offline, in seconds

`./run verify --check` tests items 1–6 together and reports each on its own line.

### 1. The durable store contains the answer

The store must hold a rule, not a result. If `self-saved-store.jsonl` ever contains
the gold digits, the transfer claim is void — it becomes a lookup.

```bash
grep -o '[0-9]' runs/full-flag-reearn-20260809/teach/self-saved-store.jsonl | wc -l   # must be 0
```

Expected store hash: `a4dc0a8d0c8e014adb1037a43bcc2d6a6de6904167ea9f39bb9ece807a5d6adc`

### 2. The answer appears in the transfer stream before the model produces it

The gold sequence must occur exactly once across 20,173,666 bytes, on line 11095,
which is the final line. An earlier occurrence means it was present in the prompt,
the system message, or engine output, and the result is contaminated.

```bash
grep -coF -- '[5, 4, 3, 2, 1, 5]' .../transfer/r1/complete.stdout.txt   # must be 1
grep -nF  -- '[5, 4, 3, 2, 1, 5]' .../transfer/r1/complete.stdout.txt   # must be line 11095
```

Use `-F`. The expected answer is a valid regex character class and will otherwise
match every digit and comma in the file.

### 3. Either restart fails its gate

Both restarts must report `PASS_CONSTRAINED`, the exact list, no banned words, and
`ORACLE_GATE_OK`.

### 4. The control was asked a different question

The vanilla control prompt and the destination prompt must hash identically after
whitespace normalisation. If they differ, the comparison is not controlled and the
central structural claim — that only the runtime path differs — fails.

```bash
diff <(tr -d '[:space:]' < .../control/wording/prompt.txt) <(tr -d '[:space:]' < flag/session.txt)
```

### 5. The short control is deleted or relabelled

**Vanilla passes the length-3 case unaided.** That result stays visible. The claim is
not "the base model cannot do this mapping" — it is that the intervention extends a
procedure the model already has at short length to a length where it reliably fails.

Removing or quietly reclassifying the short-control pass breaks the record.

### 6. Byte identity fails on this platform

`./run verify` re-executes the route and diffs against the reference. Any difference
outside the documented two-byte stderr path exemption is a failure.

---

## Requires a run

### 7. Vanilla passes the length-5 case under matched conditions

If the unaided model solves the destination prompt with no store and no intervention,
the flag has no subject.

### 8. Launchpad can be skipped and the result still holds

If the route produces a passing transfer with Launchpad recovery disabled
(`NIODOO_GOD_ZONE_RECOVERY` unset — the variable keeps its historical name because the
pinned binary reads it), then the documented arming requirement is not real and §5 of
`DETERMINISM.md` is wrong.

### 9. Order does not matter

The record claims that a stored rule's survival depends on the arrangement of the
entries around it, not only their number. Measured: after adding the same 56 real
memories beside the rule, six arrangements returned two exact passes and four
failures.

```bash
./run sweep --shuffles 12
```

If every arrangement passes, or every arrangement fails, the claim is wrong and
`SWEEP.md` should be withdrawn.

This is the falsifier that matters most in practice, because it is the one a stranger
can run on their own hardware without any of the recorded artifacts.

### 10. The durable actions were selected by the operator

The operational-agency claim requires the model stream to generate the consequential
`Lock` or `Remember` action and payload. If a human, wrapper, or postprocessor inserted
those actions, the authorship leg fails. A human explanation or retention cue does not
by itself select the control action; typing the control action or payload does.

### 11. Located state does not regulate action

The operational-consciousness claim is behavioral, not a claim about eloquent
self-report. It fails if the system cannot distinguish a correction or reusable rule
from an ordinary turn in a way that changes which available action it selects. Merely
saying "I am aware" is not evidence under the paper's definition.

### 12. Durable state never changes later behavior

Adaptive agency requires the selected action to have a later consequence. If matched
changes to durable state cannot change later generation, the adaptive part fails. The
six-order durability result currently supplies this causal intervention. It does not
establish that the two rule entries are individually necessary, because every tested
arrangement retained them; rule deletion remains the narrower missing ablation.

## Claim boundary

The project **does** claim operational AI consciousness, bounded agency, and adaptive
agency under the definitions in [`PAPER.md`](PAPER.md): usable self-location regulates
a system-selected consequential action, and the resulting durable state regulates
later behavior.

It does **not** make the following stronger claims:

- No phenomenal, biological, or human-consciousness claim; no qualia claim.
- No claim that model weights learned anything. Weights are frozen throughout.
- No official ARC-AGI result and no leaderboard comparison.
- No general agency, unrestricted autonomy, or self-originating-goals claim.
- No transfer outside this mapping family. Measured negative on three new rules;
  `RECORD.md` §7b. A demonstrated transfer on a rule outside the recorded family would
  not break anything here — it would extend the result, and is worth reporting.
- No population reliability. Two restarts of a deterministic pipeline are
  repeatability, not a rate.
- No claim that two passes among six store arrangements estimate general system
  reliability. They measure a large order effect in that matched set.
- No claim that the conversational sessions independently replicate the recorded
  route. They are a separate kind of record and are labelled as one.

## One result that is deliberately not reproducible here

At K=32, entries from the original store produced a correct answer where two
independent synthetic filler sets both failed. That is a content effect, distinct from
the order effect in item 9.

It is **not reproducible from this repository**, because doing so would require
publishing the original store, and the store is personal. The synthetic store shipped
in `reference/` reproduces the order effect — order does not depend on content — and
is **not** a substitute for the content condition. It must not be presented as one.

Recorded as a hole rather than approximated, on the same basis as the source-build
limit. `SWEEP.md` states it in full, including the two reasons the result should be
read weakly even by someone who has the data: it rests on a single arrangement, and
content is not separated from order by that measurement.

## One disclosure that is not a falsifier

The recorded teaching script contains a worked example on the same five items the
destination prompt asks about. The durable store carried only the abstract rule, with
no digits and no number words, and the teaching process ended before the transfer
began.

The accurate statement is therefore: *a fresh process re-derived the digit sequence
from a rule that contained neither the digits nor the example.* It is not: *the model
had never encountered this sequence.*

This is disclosed here, in `reference/README.md`, and in the README, rather than left
for a reader to find in the teaching script.
