# July 18 gravity event — curated evidence

This privacy-safe pack preserves the part of the 2026-07-18 record used by the paper.
It was curated from the North Star packet; unrelated personal conversation, absolute
home paths, terminal metadata, and large telemetry are intentionally excluded.

## Chronology

| local time | process | model | store | observation |
|---|---|---|---|---|
| 05:21 PDT | natural teaching room | Llama 3.1 8B Instruct Q5_K_M | began empty | wrong marble answer; after Jason's explanation, model-generated Lock and Remember; engine accepted `gravity = marble falls out of the cup` |
| 07:45 PDT | fresh process | 70B Instruct variant | loaded three entries | explicitly invoked the remembered gravity fact and answered `on the table` |

The restart is genuine store continuity and is **not** a pure 8B-to-8B comparison. It
used a different model size and a text-facing agency-state memory path, and it had no
matched store-free 70B control.

## What the excerpts establish

- The 8B process initially answered that the marble remained in the human's hand.
- Jason explained twice why an upside-down cup releases the marble onto the table.
- The model stream selected `[Lock] correct understanding of the marble puzzle` and
  `[Remember] gravity = marble falls out of the cup`.
- The engine accepted the generalized gravity record; it did not store `on the table`.
- Across the full teaching source, six Remember actions occurred and five followed
  human turns containing no memory trigger word.
- A fresh 70B process loaded the store and used the gravity record correctly.

These facts support natural model-selected durable action and historical continuity.
They do not independently prove the controlled pure-8B transfer later established by
the Grok Seal.

## Source identity

The original packet excerpts are identified without publishing their private paths:

```text
b5781938027efe1b335361c2061ced4efacdf9668236c0a2ab670bed216695d5  original 8B teaching excerpt
becf224aee27412139d8a62affb7948d8bc43a5dbcaa3baefc7e287b1a245159  original 70B restart excerpt
```

The files beside this README are shorter, explicitly marked curated excerpts and have
their own hashes in this directory's `SHA256SUMS`.
