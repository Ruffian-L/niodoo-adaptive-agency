# Local Security and Provenance Review — 2026-08-14

Scope: determine whether the restored Rust file, SSH/UFW observations, and sudden Codex usage
provide evidence of uninvited external access. This is a local evidence review, not an audit of any
vendor's server-side systems.

## Findings

- NVIDIA's system bundle installed `openssh-server` on 2026-07-23. The local SSH service was masked
  later that day. The package was purged by Jason on 2026-08-04 at 10:05 local time.
- On 2026-08-03, Avahi advertised a service named `niodoo-lumina SSH`. This can explain a visible
  “SSH started” indication without an active `sshd` listener.
- Retained system journal, authentication logs, and login records contain no SSH authentication or
  remote login. Current host state has no SSH server package, a masked/inactive SSH service, no
  port-22 listener, no `authorized_keys`, active UFW, and no externally bound TCP listener.
- A short pre-mask window on 2026-07-23 cannot be proven impossible from retained logs. The accurate
  conclusion is **no evidence of external SSH access found**, not absolute impossibility.
- Ten-minute root CRON entries are the packaged `sysstat` sampler.
- The active host check found one Codex client and its code-mode host, not a runaway Codex, Copilot,
  Cargo, Python, or Niodoo loop.
- Codex's local state shows automatic permission-review “guardian” threads. One 2026-08-10 guardian
  record accumulated approximately 6.4 million locally recorded tokens. Local recorded tokens do
  not map one-for-one to the account's weekly meter, but repeated full-context review is the strongest
  local explanation for sudden usage loss.
- `src/bin/one-shot.rs` was born before the current Sol/Codex session began. The complete pre-restore
  tool-call history for this session contains no `rm`, `gio trash`, `trash-put`, `git clean`, or
  desktop-Trash operation. The restore removed the `.trashinfo` record, so the deleting actor and
  exact deletion time cannot be attributed from remaining evidence.

## Operational boundary

Invited AI actions are treated as workflow/provenance issues, not persecution targets. The external
threat model still includes unknown remote actors and trusted-vendor compromise or overreach. Local
logs can audit commands and files visible on this host; they cannot independently verify vendor-side
data handling.
