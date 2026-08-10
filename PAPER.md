# Adaptive Agency in a Frozen 8B Model

## Abstract

We map a narrow adaptive behavior in a frozen Llama 3.1 8B Instruct model. During an earlier teaching session, the model wrote a general list-mapping rule to a durable store. After the process ended, two new processes loaded the same store and answered a differently worded task from the same ARC-style family. Both produced the exact constrained answer `[5, 4, 3, 2, 1, 5]`. The flagged path used residual representations derived from the stored rule and sparse affinity to rule-related tokens. It did not update model weights, inject the stored rule as prompt text, or install a gold digit sequence. A vanilla `llama.cpp` route using the same model bytes took wrong routes on the original and changed-wording length-five tasks, reached a shorter destination, and took a wrong route on a letter-token case. The flag is adaptive agency under this explicit loop-breaker definition: teach scar, process death, exact same-family wording transfer.

## Operational definition

For this report, adaptive agency requires all of the following:

1. Teaching produces a durable rule record without an operator command to save the numeric answer.
2. The teaching process ends.
3. A new process receives a differently worded task from the same mapping family.
4. The process uses the durable record without inserting its text into the prompt.
5. The output is exactly correct under the task constraints.
6. Direct gold-sequence construction paths are excluded mechanically.

This definition is narrower than general agency and broader than ordinary in-context completion because the relevant state crosses a process boundary through a durable store.

## Task

The examples define the mapping

$$
[x_1, x_2, \ldots, x_n] \mapsto [x_n, x_{n-1}, \ldots, x_1, x_n].
$$

In plain words: start at the end, walk through the list to the start, then repeat the original end item. The destination prompt asks for the mapping on `[1, 2, 3, 4, 5]` while prohibiting three common description words. The correct result is `[5, 4, 3, 2, 1, 5]`.

## System path

The model weights remain frozen. The durable store contains a textual rule scar but not the numeric gold answer. In flag mode, Niodoo converts the best stored scar into residual ears and applies a dual-stream intervention during generation. Sparse logit affinity is limited to procedure anchors such as start, end, first, and last. Prompt reinjection is disabled. Gold-order boost and stop boost are zero. Procedure force-emission and direct progress-digit tips are off.

The flag settings were residual mass 5, dual inject gain 1.0, dual posture boost 8, and scar-token logit boost 1.2. The decode used temperature 0, layers 16 through 33, theta override 1.5, physics blend 0.9, and at most 768 steps.

## Results

Both independent restarts produced `[5, 4, 3, 2, 1, 5]`, used none of the prohibited words, and passed the mechanical gate for excluded gold-construction paths.

The pure vanilla control used the same Q5_K_M model bytes through `llama-cli`, no system prompt, temperature 0, and seed 42. It returned `[1, 3, 4, 5, 2]` on the original prompt and `[5, 4, 2, 1, 3]` on the changed wording. It correctly returned `[3, 2, 1, 3]` on the shorter case and failed the letter case with `[B, C, D, E, A, B, C, D, E]`.

## Coordinates and milestone

A stronger dual intervention, inject 1.5 and posture 12, nearly completed the mapping but returned `[5, 4, 3, 2, 2, 5]`. That coordinate located the remaining instability. More scar mass destabilized generation, while removing procedure clauses lost the end-to-start structure. The milestone was softening the dual intervention to inject 1.0 and posture 8. That boost refueled the stored geometry without a direct next-digit path.

## Interpretation

The result shows that a model-mediated rule record can persist beyond one process and influence exact behavior under changed wording. The process boundary distinguishes the result from a single context window. The excluded-path gate distinguishes it from directly supplying the stored rule or desired digits at generation time.

Why every token was selected, broad robustness, and human-like agency remain unmapped. The assistant explanations also contain imperfect positional language even when the final list is exact. The recorded loop marks behavior, not consciousness or unrestricted autonomy.

## Limitations

- One model, one quantization, one mapping family, and one main destination prompt.
- Two arrivals locate repeatability here; they do not estimate statistical reliability.
- The teaching event and destination are linked by the frozen store and decision trail; the lean public batch is not a single continuous video capture.
- The original literal shell command was not retained, though the full settings and byte identities were.
- The engine and binaries are external to this map repository.
- This is not official ARC-AGI and should not be compared to an ARC leaderboard score.

## Map availability

The exact human-readable prompts, model replies, scores, vanilla coordinates, store, and settings are included. `cargo run --locked` checks the map without downloading a model or executing either inference runtime.
