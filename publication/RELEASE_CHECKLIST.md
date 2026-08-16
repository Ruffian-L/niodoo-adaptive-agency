# Release Checklist

## Repository

- [x] `cargo fmt --check`
- [x] `cargo test --locked`
- [x] `cargo run --locked`
- [x] `SHA256SUMS` matches every published file
- [x] `reference/SHA256SUMS` verifies
- [x] No private paths in tracked files (`.env.local` is gitignored)
- [ ] No model weights, tokenizer, binaries or secrets committed
- [ ] Clean-clone offline verification succeeds

## Tooling

- [x] `manifest.toml` is the only file recording a URL, hash or size
- [x] `./run` with no argument lists the subcommands and nothing else
- [x] `./run doctor` passes on the reference machine, exits 2 with a named cause otherwise
- [x] `./run fetch` is idempotent by hash; second run reports `already present, hash ok`
- [x] `./run verify --check` passes 6/6 offline with no GPU
- [x] `./run verify --check` exits 1 and names expected vs actual on a corrupted reference
- [ ] `./run docs-check` passes title, claim, attribution, link, and evidence-hash checks
- [ ] `./run verify` full re-execution passes on the reference machine
- [ ] `./run chat` starts, writes to the store, and resumes
- [ ] `./run sweep` reports a pass rate across arrangements using the bundled synthetic store
- [x] Exit codes: 0 pass, 1 mismatch, 2 environment, 3 missing artifact

## Documentation

- [x] `README.md` states the short-control pass, repeatability-not-a-rate, the order
      effect, frozen weights, and the binary-level boundary — all above the fold
- [ ] The canonical “Knowing Where You Are” title matches the paper, citation,
      Zenodo, and dataset card
- [ ] Current docs claim operational AI consciousness while excluding phenomenal and
      human consciousness
- [ ] Grok Seal and Jason & Sol Seal are named as mnemonic, non-ownership labels
- [ ] The 56-added-memory result is reported as two of six arrangements, not a third
      seal, fixed capacity, or general reliability rate
- [ ] Jason's sole authorship and the non-hierarchical collaborator contribution rule
      are consistent across credits and publication metadata
- [ ] Historical planning documents carry dated status banners rather than rewritten
      history
- [x] `FALSIFIERS.md` linked from the first screen
- [x] `DETERMINISM.md` records every setting the byte-identity claim depends on
- [x] `RECORD.md` carries the observations with sources
- [x] `reference/README.md` discloses the teach-script worked example and the
      short-control pass rather than leaving them to be found
- [x] `SWEEP.md` states the content result as a named hole and does not substitute
      synthetic data for it
- [x] No anthropomorphising register in shipped documentation
- [ ] Every internal link resolves
- [ ] Curated July and durability evidence-pack hashes verify
- [ ] No personal memory payloads or absolute home paths appear in published evidence

## Assets to publish

- [ ] Niodoo runtime — 90,104,160 bytes, `2151c184…78b507`
- [ ] `llama-cli` build `c0bc859` — 74,261,680 bytes, `72c08ab8…4b4b5d`
- [ ] Ghost-basin registry — 24,138 bytes, `6e361f83…fa747b`
- [ ] Reference run tarball — 224,455,846 bytes uncompressed
- [ ] Fill `url`, `sha256` and `bytes` for each in `manifest.toml`
- [ ] Re-run `./run fetch` on a clean clone to confirm every URL resolves

## Publication

- [ ] Create the public GitHub repository
- [ ] Add the repository URL to `REPRODUCE.md` and `CITATION.cff`
- [ ] Tag an immutable release
- [ ] Upload the tagged archive to Zenodo and record the DOI
- [ ] Publish the Hugging Face dataset card and archive
- [ ] Record the demonstration without implying an old run was captured continuously
- [ ] Add DOI, video URL and Hugging Face URL to `README.md`

## Known open items, to state rather than fix silently

- **Build from source is not supported.** A clean build at the recorded product
  revision does not compile. Reproduction is binary-level and `DETERMINISM.md` §2
  says so. `verify --from-source` is the next milestone, not part of this release.
- **A `teach` lane was built and cut.** Parameterising the route to teach an
  arbitrary rule worked mechanically. Three new rule families produced no transfer;
  the result is recorded in `RECORD.md` §7 as a stated boundary. Shipping three
  failing examples as the headline command would have been worse than not shipping
  the command, so the harness was reverted byte-identical and the presets removed.

  The control run for that revert re-executed the recorded task and matched the
  reference byte for byte. Stated precisely: **the engine binary was the prebuilt
  pinned one throughout (`2151c184…`, dated 2026-08-09), and the edits were
  harness-level.** Byte-identity is therefore the expected outcome and is evidence
  that harness changes did not perturb the route — not evidence about the compute
  path or the source. What it does establish: two cold reproductions on separate
  days, and that the harness can change without touching the result.
- **The store-content effect is not reproducible from this repository.** See
  `SWEEP.md`. Synthetic data reproduces the order effect and is explicitly not a
  substitute for the content condition.
