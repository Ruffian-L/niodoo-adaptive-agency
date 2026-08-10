# Setup Lanes

## Portable map check

This lane checks the archived map on any system with Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo run --locked --bin niodoo-adaptive-agency-map
```

When a prebuilt map-checker release asset is published, Rust will no longer be required for this lane.

## Exact inference lane

The flagged binary is not cross-platform. Its current lane is:

- Linux `aarch64`
- NVIDIA GB10-class GPU
- NVIDIA driver compatible with CUDA 13
- CUDA 13 libraries: `libcuda`, `libnvrtc`, `libcurand`, `libcublas`, `libcublasLt`, and `libcudart`

`cargo one-shot` automatically downloads and verifies the 5.4 GB bartowski model and the byte-identical NousResearch tokenizer mirror when absent. Both unattended URLs were checked on 2026-08-09. It also verifies the Niodoo binary, llama.cpp build, tokenizer, and ghost-basin registry.

The Niodoo and llama.cpp binary assets are not published yet. Until they are uploaded as hash-pinned GitHub Release assets, a new machine must provide them through `ONE_SHOT_NIODOO_BIN` and `ONE_SHOT_LLAMA_CLI`. The runner stops rather than substituting an unverified binary.

The recorded product revision `9de966d` is not presently a clean source-build path. A detached build at that revision exposed tracked references to Qwen/cache and hook code that was untracked or absent from the commit. The historical Niodoo binary is hash-pinned, but its complete source state was not committed. Do not advertise build-from-source reproduction until that source state is reconstructed and frozen.

This is an open release milestone, not a completed portability flag.
