# Release Checklist

## Repository

- [x] `cargo fmt --check`
- [x] `cargo test --locked`
- [x] `cargo run --locked`
- [x] `SHA256SUMS` matches every published file
- [ ] No model weights, tokenizer, binaries, secrets, or private paths beyond the historical byte trail
- [ ] Clean-clone offline verification succeeds

## Map

- [ ] Flag card prints the correct answer beside every result
- [ ] Climb card includes the route, plain-language math, decision note, and sign-off
- [ ] Both assistant transcripts are readable
- [ ] Vanilla failures and short-case pass are equally visible
- [ ] Map edge says one model and one mapping family

## Publication

- [ ] Create GitHub repository `niodoo-adaptive-agency`
- [ ] Add public repository URL to `REPRODUCE.md` and `CITATION.cff`
- [ ] Tag an immutable release
- [ ] Upload the tagged archive to Zenodo and record the DOI
- [ ] Publish the Hugging Face dataset card and archive
- [ ] Record and publish the demonstration without implying an old run was captured continuously
- [ ] Add DOI, video URL, and Hugging Face URL to the README
