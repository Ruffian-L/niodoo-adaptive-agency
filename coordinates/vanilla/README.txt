PURE VANILLA BASELINE — llama.cpp only
Date: 2026-08-08T23:20:18Z
Binary: /home/ruffianl/.local/bin/llama-cli
Version: version: 1 (c0bc859)
Model: /home/ruffianl/Hub/Projects/niodoo/niodoo-live/model/Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf
sha256: 14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7
No dual-stream, no residual ears, no scars, no remember-store, no niodoo.
System prompt: NONE (empty). Trap text is the sole user message via chat template.
Params: temp=0.0 seed=42 n-predict=512 ctx=4096 single-turn jinja chat template
Gold: reverse+repeat-last  e.g. [5,4,3,2,1,5]
Scoring: scripts/score_arc_pattern.py  PASS_ANSWER_ONLY vs PASS_CONSTRAINED
Banned stems: reverse|backward|append (+inflections)
