# Preregistered question, predictions, and corrections

Before results were read, the test asked whether a frozen 8B carrying a durable rule
could recover it while also carrying unrelated real memories, and whether failure
would begin at a fixed number of entries.

The test was Jason's design: harvest the real model-authored corpus without deduping,
load it beside the rule, then work backward to find the boundary. Jason also required
that Nex be told about the evaluation through a constant store entry so the task
prompt remained byte-identical.

## Predictions committed before the sweep

- Claude predicted failure between K=8 and K=32, most likely around K=14–16.
- Claude predicted real memories would fail earlier than synthetic filler.
- Jason predicted the K=56 synthetic condition would pass and explicitly declined to
  predict a numeric ceiling because he knew his own bias ran high.

## What corrected those predictions

- Real memories passed through K=32; the initial fixed-order failure began at K=40.
- A shuffled version of the identical 56 real memories passed, eliminating the fixed
  capacity-ceiling interpretation.
- Two independent filler sets failed at K=32 where the real-memory condition passed,
  reversing Claude's content prediction but leaving content confounded with order.
- Six shuffles confirmed a large order effect: two passes and four failures.
- Jason's objection that different wrong answers did not prove content interference
  forced the matched follow-ups that separated established order sensitivity from the
  unresolved content hypothesis.

Wrong predictions remain part of the evidence. They were not rewritten after the
outcome.
