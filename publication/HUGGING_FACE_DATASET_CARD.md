---
pretty_name: Adaptive Agency in a Frozen 8B Model
license: other
task_categories:
  - text-generation
language:
  - en
tags:
  - persistent-memory
  - hidden-state-steering
  - reproducibility
---

# Adaptive Agency in a Frozen 8B Model

This dataset contains prompts, model replies, scores, decision records, and an offline map checker for one operationally defined adaptive-agency destination.

## Contents

- The two historical Niodoo arrivals on a changed-wording task.
- The frozen rule store and flag settings.
- Four vanilla `llama.cpp` controls using the same model bytes.
- Human-readable flag and climb cards.
- The 2026-08-09 continuous one-shot coordinate, which reached autonomous save but not fresh transfer.

## Scope

The demonstrated scope is store-mediated cross-restart transfer within one ARC-style mapping family. It is not official ARC-AGI, a model release, or a broad agency benchmark.

## Licensing

Repository code and original documentation are MIT. Model outputs were generated with Meta Llama 3.1 and may remain subject to applicable upstream terms; therefore the dataset card uses `license: other`. No model weights or tokenizer files are included.

## Verification

Run `cargo run --locked` from the repository root.
