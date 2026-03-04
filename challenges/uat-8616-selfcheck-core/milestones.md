---
challenge_id: uat-8616-selfcheck-core
---

# Milestones — UAT-8616 selfcheck-core

## Progression

| Date | Milestone | Evidence |
|------|-----------|----------|
| 2026-02-25 | **Incited** — Joint disclosure by CISA, ASD/ACSC, CCCS, CSA Singapore | CVE-2026-20127, CVE-2025-20393 |
| 2026-02 | **Claimed** — Base forensic detection kit developed | `toolkit/src/asyncos.rs` (layers 0–5b) |
| 2026-03 | **Harnessed** — 51 Python mirror tests passing | `test_harness.py` |
| 2026-03 | **Ongoing** — Active threat, independent review requested | [DISCLOSURE.md](DISCLOSURE.md) |

## Takeaways

### Confirmed
- **Stealth-normalized forensic I/O is necessary:** These threat actors actively monitor devices for forensic activity. The torque module's I/O normalization is not paranoia — it addresses a documented adversary capability.
- **Cleanup creates artifacts:** Firmware downgrade-and-restore, log surgery (AquaPurge), and implant hiding all leave residual traces detectable by z-score deviation from known-good baselines.
- **Single-dependency crate is auditable:** 8,453 lines of Rust with one dependency (anyhow). The entire codebase is human-readable in a single sitting.

### Open for Audit
- **Base kit correctness (layers 0–5b):** IOC matching against known indicators. Independent review requested — see [DISCLOSURE.md](DISCLOSURE.md).
- **False positive characterization:** The base kit has not been tested against a clean appliance population to establish false positive rates.
- **Hunt Guide rule coverage:** 3 of N ACSC Hunt Guide rules mapped. Coverage completeness unknown.

### Open for Application
- **Expansion layer promotion:** Individual experimental layers (6–12) may be promoted to review scope as their theoretical foundations are validated. See `expansions/README.md`.
- **Cross-platform adaptation:** The detection architecture (stealth I/O + layered forensic analysis + IOC sidecar updates) could be adapted for other infrastructure appliance families under active exploitation.
- **Constraint harness integration:** Building a standard 3-bridge constraint harness for this challenge would validate the hypothesized archetype edges to TVN-Payne and AoC.
