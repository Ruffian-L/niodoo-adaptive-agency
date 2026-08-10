# Full-Tools Merged-Route Preflight

**Status: PASS engineering preflight. This is not the earned organic run and not a new flag.**

Run it with:

```bash
cargo merged-preflight
```

The runner fails closed unless the pinned model, tokenizer, and new-coordinate Niodoo binary hashes match. It then:

1. asks Niodoo to print the exact resolved free-mode system-prompt bytes before model loading;
2. gives those exact bytes to a matched `llama.cpp` control;
3. starts Niodoo with the same prompt, the full `list/read/write/memory_status/recall` tool surface, a new tool home, and a new Remember store;
4. forces the old Niodoo Qdrant vault tether off, explicitly enables loopback SplatRAG, and removes every inherited `NIODOO_*` setting before applying the isolated settings; and
5. retains complete subprocess stdout and stderr without rewriting model prose.

The historical flag runner and pinned binary are unchanged. This runner uses a separate binary coordinate:

```text
Niodoo SHA-256: 94c6970ee36f44f65dea4a5dd922d7264002bc1ec1c8d34bfec0d559b79171c3
Resolved full-tools prompt SHA-256: d5589b7af54afe18fd69f0cd74a0574ee28bd2a321ffd28c0752e2944d530f70
```

## Passed run

The current passed repair run is [runs/merged-preflight-1786335572471193642-4122245/RUN_MAP.md](runs/merged-preflight-1786335572471193642-4122245/RUN_MAP.md).

- Exact prompt receipt: [prompt-receipt/RECEIPT.txt](runs/merged-preflight-1786335572471193642-4122245/prompt-receipt/RECEIPT.txt)
- Exact resolved prompt bytes: [resolved-system-prompt.txt](runs/merged-preflight-1786335572471193642-4122245/prompt-receipt/resolved-system-prompt.txt)
- Matched llama raw conversation: [complete-conversation.txt](runs/merged-preflight-1786335572471193642-4122245/llama-control/complete-conversation.txt)
- Matched llama receipt, including the correct answer: [llama-control/RECEIPT.txt](runs/merged-preflight-1786335572471193642-4122245/llama-control/RECEIPT.txt)
- Niodoo raw stdout: [complete.stdout.txt](runs/merged-preflight-1786335572471193642-4122245/niodoo-tool-smoke/complete.stdout.txt)
- Niodoo raw stderr and engine receipts: [complete.stderr.txt](runs/merged-preflight-1786335572471193642-4122245/niodoo-tool-smoke/complete.stderr.txt)
- Tool gate: [GATE.txt](runs/merged-preflight-1786335572471193642-4122245/niodoo-tool-smoke/GATE.txt)
- Model-authored durable store: [isolated-remember-store.jsonl](runs/merged-preflight-1786335572471193642-4122245/niodoo-tool-smoke/isolated-remember-store.jsonl)

The matched llama control did not produce the correct `[5, 4, 3, 2, 1, 5]`. The system prompt therefore did not turn the vanilla lane into a prompt-engineered pass.

During the mechanical tool smoke, Niodoo's generated stream authored `write` and `read` calls and also emitted its own `<remember>` and `<lock>` tags. The engine executed the tools and reported the durable Remember acceptance. Those lines are model output, not wrapper narration. This is useful plumbing evidence, but the deliberately direct smoke request is not presented as the organic gravity/ARC event.

## Preserved failed preflights

Two failed coordinates remain intact:

- [merged-preflight-1786317138827492828-2](runs/merged-preflight-1786317138827492828-2/RUN_MAP.md): sandboxed llama could not access the GPU/temporary loopback port.
- [merged-preflight-1786317168991014101-4068371](runs/merged-preflight-1786317168991014101-4068371/RUN_MAP.md): tools worked, but the first strict path assertion expected a different relative path.
- [merged-preflight-1786317277437638122-4068862](runs/merged-preflight-1786317277437638122-4068862/RUN_MAP.md): exact tool round-trip succeeded and Niodoo authored a durable Remember; the old gate incorrectly required its isolated store to stay empty.

None of these outputs were relabeled or cleaned after the fact.

## Source and build provenance

The new binary was built from the live working tree at Git `9de966d2e65c7ce9252e98f41754d030535b2124`. That tree was dirty (`204` porcelain entries), so the commit alone is not a reproducibility claim. Relevant build-input bytes were:

```text
1ee6b9817dc53b2fbf2ac964cd41581886b1bea8520bbd4e87290e00b1de9742  niodoo/src/openai_tools.rs
1795a491bd75230e8857f4c80adc55c5cc41bbf82910c75202328df903777bcd  niodoo/src/main_helpers2.rs
b228a072435a9d2f15640672b8b808f5e0e76edbd7a4e60cd04afd037b6caeaf  niodoo/src/tests.rs
ac3d393bbccd129f719441d5191294c99d9cdedb443e7a0d25492762ca519ffa  niodoo/src/cli.rs
17cd08f526e3e8dd55e1a86c1434f2a8efdf1e31acc68d6e1b6a61b063766490  niodoo/src/main.rs
df2d9e2e681275efcf1a302bb89c075e92b6b17aea542359cebceed933d0648e  niodoo/Cargo.toml
5536d57c4031b7cd4bcbdf1d247ed30c1105752e86dc4cd630ffda4d4049d872  niodoo/Cargo.lock
30ea2360b847acf1329c2105d6cfbccf94d56dc7f9be1e6c7a58bb92d8eb37d1  niodoo/build.rs
```

Build command:

```bash
CUDA_COMPUTE_CAP=121 RUSTFLAGS='-C target-cpu=native' \
  cargo build --release --locked --features niodv4_bridge --bin niodoo \
  --target-dir /home/ruffianl/projects/niodoo-adaptive-agency/.one-shot/target-niodoo-tools
```

The next real step remains user-operated natural conversation. An earned merged result requires the rare solve, the human's natural explanation of why, a model-authored and engine-confirmed durable Lock or Remember, process death, and exact same-family wording transfer. The preflight does not automate or pre-script that event.

Start that separate natural room with:

```bash
cargo merged-live
```

The human-operated room defaults to the full Llama 3.1 context ceiling of `131072` tokens. The
cheap mechanical preflight remains at `8192`; it no longer controls the live room. An exceptional
live override must be explicit, for example `MERGED_LIVE_CONTEXT_LENGTH=65536 cargo merged-live`,
and the effective value is written into the run receipt.

The live tool surface now includes `memory_status` and `recall`, backed by the loopback-only
SplatRAG service at `127.0.0.1:8767`. This is separate from the old 64D Remember-vault tether,
which remains forced off. A compact model spelling such as `<read>{"path":"notes/a.md"}</read>`
is normalized into a real tool call; invalid JSON produces a visible parse-error response rather
than an unverified claim that a file was read.

The live runner creates a new isolated tool home and Remember store, captures raw stdout and stderr, and waits for the human's unscripted opening. It does not insert the gravity prompt, ARC prompt, teaching explanation, or a request to save anything.

It also records the human's exact stdin bytes as `complete.stdin.txt` and writes a lightweight `compact-resume-state.json` when the room exits. To carry a prior room forward without modifying its artifacts:

```bash
MERGED_LIVE_RESUME=/absolute/path/to/runs/merged-live-... cargo merged-live
```

The resumed room gets a new directory. The runner copies the prior durable Remember store and isolated tool workspace into it, and loads the prior compact-resume state when one exists. This is continuity with explicit provenance; it is not described as an identical KV tensor. Rooms created before input capture and compact resume were added can still carry their Remember store and workspace forward, but their human side must be preserved from the terminal separately.

For a process-death transfer gate, load only the model-authored durable store and explicitly exclude the prior workspace and compact conversation state:

```bash
MERGED_LIVE_RESUME=/absolute/path/to/runs/merged-live-... \
MERGED_LIVE_COLD_STORE_ONLY=1 \
cargo merged-live
```

The new run receipt records `resume_scope=DURABLE_REMEMBER_STORE_ONLY`.
