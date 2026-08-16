# Trust the Bytes

| Artifact | SHA-256 or revision |
|---|---|
| `Meta-Llama-3.1-8B-Instruct-Q5_K_M.gguf` | `14e10feba0c82a55da198dcd69d137206ad22d116a809926d27fa5f2398c69c7` |
| Llama 3.1 tokenizer | `79e3e522635f3171300913bb421464a87de6222182a0570b9b2ccba2a964b2b4` |
| Niodoo flag binary | `2151c1840bb21f1cc688b49a704c14670ea12d806113b06d7f212eb19278b507` |
| Niodoo flag binary size | `90104160` bytes |
| Niodoo map tree | `8033ec2` |
| Niodoo product tree | `9de966d` |
| Paper revision | `07bb87b` |
| Original decision flag commit | `9e1bd2e` |
| Vanilla `llama-cli` | build `b1-c0bc859` |

The model, tokenizer, engine, and binaries are not redistributed here. `SHA256SUMS` pins every file that is redistributed in this repository.

## Source-state edge

The Niodoo binary hash is stronger than the recorded product revision for this route. A clean detached build of `9de966d` does not compile without source files or interfaces that were outside that commit. Until the exact dirty build state is reconstructed, treat the binary hash as the executable identity and the revision as an incomplete source coordinate.
