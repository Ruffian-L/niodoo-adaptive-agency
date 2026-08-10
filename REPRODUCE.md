# Revisit the Map

## One command: control first, then flag

On the original machine:

```bash
cargo one-shot
```

See [ONE_SHOT.md](ONE_SHOT.md) for path overrides and the fail-closed checks.

## Level 1: check the published map

Requirements: a Rust toolchain. No model, GPU, Python environment, or network access is needed.

```bash
cd niodoo-adaptive-agency
cargo run --locked
```

Run those commands after cloning or downloading the public repository.

Expected output:

```text
FLAG HELD: destination and four vanilla coordinates mapped
```

This checks the human-readable map and route conditions. It does not rerun inference.

## Level 2: rerun the vanilla control

Requirements:

- `llama-cli` build `b1-c0bc859` or a documented replacement.
- Model file with SHA-256 `14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7`.
- The prompts under `coordinates/vanilla/<variant>/trap.txt`.

Canonical command:

```bash
llama-cli -m "$MODEL" \
  --system-prompt "$(cat route/system_prompt.txt)" \
  -p "$(cat coordinates/vanilla/original/trap.txt)" \
  -n 512 --temp 0.0 --seed 42 -c 4096 -ngl 99 -st --jinja \
  --no-display-prompt
```

Save and score assistant-only output as a new matched-system coordinate. Shell command substitution removes the text file's final newline, matching the live Rust runner. The archived control includes full stdout, cleaned assistant text, and both noisy and assistant-only scores for transparency, but remains the earlier no-system coordinate.

## Level 3: map a fresh Niodoo coordinate

This repository intentionally does not ship the engine or binary. A comparable rerun requires:

- Niodoo product commit `9de966d` or a separately documented port.
- Binary SHA-256 `2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507` for byte-identical comparison.
- The pinned model and tokenizer bytes in `TRUST_THE_BYTES.md`.
- The route settings in `CLIMB_CARD.md` and `flag/flag_settings.txt`.

The exact literal shell command was not retained. A fresh public run must create a new dated coordinate, record its full command, and identify every binary, source, hardware, or setting difference. A new arrival may plant a new flag; a different route is welcome.
