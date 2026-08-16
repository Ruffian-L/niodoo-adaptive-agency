# reference/

Expected outputs from the recorded run, small enough to live in git. Everything here
can be read and checked with no download, no GPU and no model.

`SHA256SUMS` pins every file. Check it with:

```bash
cd reference && sha256sum -c SHA256SUMS
```

| File | What it is |
|---|---|
| `self-saved-store.jsonl` | The durable store written during teaching. Two lines, no digits. |
| `teach-script.txt` | The human turns fed to the teaching process. See the disclosure note below. |
| `transfer-r{1,2}-score.json` | Authoritative scorer output for both restarts. |
| `transfer-r{1,2}-oracle-gate.txt` | Excluded-path gate result for both restarts. |
| `transfer-r{1,2}-final-line.txt` | The last line each restart produced. |
| `transfer-r1-run-timing.json` | Decode token count, stop reason, device. |
| `control-wording-prompt.txt` | The prompt given to the unaided control. |
| `control-{original,wording,short,letters}-score.json` | All four control results, including the one that passes. |

## Two things to notice here

**`control-short-score.json` is a pass.** The unaided model solves the length-3 case.
Both length-5 failures and this pass are kept together on purpose — the claim is that
the intervention extends a procedure the model already has at short length, not that
the base model cannot do the mapping.

**`teach-script.txt` line 4 contains a worked example on the same five items** the
destination prompt asks about. The durable store carried only the abstract rule, with
no digits and no number words, and the teaching process ended before the transfer
began. The accurate statement is that a fresh process re-derived the digit sequence
from a rule containing neither the digits nor the example — not that the sequence had
never been encountered. This is disclosed here rather than left to be found.

The full 215 MB run — every byte of stdout, stderr and telemetry for every phase — is
a separate download. See `manifest.toml`.
