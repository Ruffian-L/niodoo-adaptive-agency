# Recorded Demonstration Plan

## Act 1: establish the control

1. Show the repository revision and model SHA-256.
2. Run vanilla `llama-cli` on the changed-wording prompt with no system prompt.
3. Show its actual answer `[5, 4, 2, 1, 3]` beside the correct answer `[5, 4, 3, 2, 1, 5]`.
4. Show the short-case vanilla pass so the control is represented honestly.

## Act 2: establish persistence

1. Begin from an empty durable store in a fresh, separately documented run.
2. Record the teaching conversation and spontaneous rule write in full human-readable form.
3. Display the resulting general rule record and confirm that it contains no numeric gold list.
4. Show the process ID, terminate the process, and verify that it is no longer running.

## Act 3: transfer after restart

1. Start a new process with the frozen store and changed-wording prompt only.
2. Show the exact output and run the constrained scorer and excluded-path gate.
3. Repeat from a second new process.
4. Run `cargo run --locked` and show the final flag card.

The video must label a fresh run as a new coordinate. It must not splice the 2026-08-08 teaching event and destination batch into the appearance of one continuous capture.
