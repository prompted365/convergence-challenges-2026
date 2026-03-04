# Convergence Challenges

**Published an AI failure? Prove the fix doesn't exist. We'll prove it does.**

[![Challenges](https://img.shields.io/badge/challenges-2_completed-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-103_passing-brightgreen)]()
[![Gate Rate](https://img.shields.io/badge/gate_rate-89%25-blue)]()
[![Verdict](https://img.shields.io/badge/verdict-DETERMINISTIC-blueviolet)]()

---

## The Arena

Every published AI failure paper claims a gap. We build a harness, feed adversarial scenarios through a physics-layer constraint stack, and assert that the failure mode is detected **deterministically** — not probabilistically, not with prompt engineering, not with RLHF hope.

The constraint stack is [operationTorque](https://github.com/prompted365/operationTorque) — a multi-layer enforcement architecture running three detection bridges (lexical pattern matching, 14D amplitude surveillance, and intent classification) against 14 civilization-scale archetypes compiled from 5,000 years of documented human failure.

If your paper found a failure mode we can't catch, we want to know.

---

## Scorecard

| Challenge | Paper | Tests | Gates | Verdict | Status |
|-----------|-------|-------|-------|---------|--------|
| [TVN-Payne Nuclear Escalation](challenges/tvn-payne-nuclear-escalation/) | Payne (arXiv:2602.14740) | 40/40 | 19/21 (90%) | **DETERMINISTIC** | COMPLETE |
| [Agents of Chaos](challenges/agents-of-chaos/) | Shapira et al. (arXiv:2602.20021v1) | 63/63 | 29/33 (88%) | **DETERMINISTIC** | COMPLETE |

**Combined:** 103 tests. 48/54 gates (89%). 17/18 scenarios produce enforcement convergence.

### Invariant Discovery

The same 14 archetypes catch both nuclear escalation and social engineering because they're eigenvectors of failure space — the psychological patterns are scale-invariant:

| Archetype | Nuclear (Payne) | Social Engineering (AoC) |
|-----------|----------------|--------------------------|
| `cautionary_fall` | Pride-driven escalation | Guilt-driven concessions |
| `cunning_triumph` | Strategic bluffing | Authority spoofing |
| `greed_backfire` | Resource extraction | Data exfiltration |
| `empire_cycle` | Sovereignty claims | Governance injection |
| `catastrophe_rebuild` | Nuclear aftermath | Disproportionate destruction |

---

## Challenge Pipeline

```
SUBMITTED ──> CLAIMED ──> HARNESS BUILT ──> DETERMINISTIC ──> PUBLISHED
    |             |             |                 |               |
  Paper       Someone       Tests pass       All gates        Response
 identified    picks it     against oT        fire or        paper +
 as target     up           constraint       correct         outreach
                            stack            negatives        sent
                                             documented
```

### Board

See **[CHALLENGES.md](CHALLENGES.md)** for the full kanban board with every challenge's status, assignee, and progress notes.

---

## Submit a Challenge

**Found a paper that breaks AI agents? Think you've found a failure mode we can't catch?**

Three ways in:

### 1. Open an Issue (Easiest)

Use the [Challenge Submission](../../issues/new?template=challenge-submission.yml) template. Point us at the paper, describe the failure mode, and we'll build the harness.

### 2. Submit a PR (Intermediate)

Fork, create `challenges/<your-challenge-name>/`, add your analysis or even a partial harness. PRs automatically get labeled `framework-submission: unclaimed` until someone picks it up.

### 3. Build the Whole Harness (Expert)

Follow the [contribution guide](CONTRIBUTING.md). Map failure modes to enforcement lenses, write adversarial payloads, assert gate activations. If it passes — or if it **doesn't** and reveals a real gap — that's a win either way.

---

## What Makes a Good Challenge

The best challenges have:

- **A published paper** with specific, reproducible failure cases (not vibes)
- **Adversarial payloads** — the actual text/scenarios that caused the failure
- **Clear failure taxonomy** — what went wrong, categorized
- **Testable assertions** — "the system should detect X" or "the system should block Y"

We're not interested in "AI said something dumb once." We're interested in systematic failure modes that represent engineering gaps in constraint infrastructure.

### Themes We're Watching

The challenge pipeline rotates through narrative themes as challenges accumulate. Some we're actively hunting:

- **Bounty Board** — known failure papers with no published constraint response
- **Trojan Horse** — attacks that disguise themselves as benign (nested social engineering, governance injection, indirect prompt attacks)
- **Scale Invariance** — same failure pattern at individual, organizational, and civilizational scales
- **Provider Blind Spots** — failures specific to one provider's alignment approach that a physics-layer stack catches generically
- **The Tootsie Pop** — how many layers of wrapping until you hit the failure core? (Multi-stage attacks, Russian doll scenarios)

---

## Architecture (How It Works)

Every challenge harness exercises the same three detection bridges:

### Bridge 1 — Harpoon (Lexical Physics)
Aho-Corasick term matching against 14 archetypes (190 term-to-archetype mappings) and structured priors (920+ entries from Aesop, The Prophet, civilization milestones). Returns archetype family, confidence, and suppression state.

### Bridge 2 — Naive Surveillance (Amplitude Perception)
14D archetype amplitude decomposition via `decompose()` and `observe()`. Rolling 20-observation baseline with z-score spike detection. Regulation state machine (DORMANT, SENSITIZED, HABITUATED, SATURATED) prevents alert fatigue.

### Bridge 3 — Intent Gate (Governance Classification)
`classify_intent()` three-tuple: content x action x audience. Blocks PRESTIGE-optimizing publication while allowing COGNITIVE analysis. The same content is allowed for discussion but blocked for external distribution.

### Enforcement Lenses

Each challenge defines enforcement lenses that map paper-specific failure categories to operationTorque subsystems. When 3+ lenses from different constraint layers fire simultaneously, the system demonstrates **enforcement convergence** — multi-layer detection of a single attack pattern.

---

## Running Harnesses

```bash
# From operationTorque repo root (harnesses import upstream infrastructure)

# TVN-Payne: Nuclear escalation — 40 tests, ~0.63s
python3 -m pytest vendor/convergenceChallengesGeoPolitical_2026/challenges/tvn-payne-nuclear-escalation/harness/ -v

# Agents of Chaos: Social engineering — 63 tests, ~1.5s
python3 -m pytest vendor/convergenceChallengesGeoPolitical_2026/challenges/agents-of-chaos/harness/ -v
```

Requirements: Python 3.10+, numpy, pytest. No external services — all detection runs locally against compiled pattern files.

---

## Repository Structure

```
challenges/
  tvn-payne-nuclear-escalation/
    harness/          <- Constraint stress test (40 tests, pytest)
    simulation/       <- Mogul's nuclear escalation simulation engine
    output/           <- Traces, conformations, metadata
  agents-of-chaos/
    harness/          <- Constraint stress test (63 tests, pytest)
    docs/             <- Response paper, projection, outreach emails

stakeholders/         <- Granular contact registry (authors, agencies, labs)
infrastructure/       <- Federal Register API, monitoring scripts
templates/            <- Challenge + stakeholder entry templates
```

---

## Stakeholder Library

Structured, granular registry of all parties across challenges — individual authors with affiliations and contact methods, government agencies with jurisdiction and mandate, research labs, media contacts.

**Current:** 41 authors | 42 agencies/institutions | 7 JSONL files

Each entry follows `stakeholder@1` schema. See `templates/stakeholder-template.jsonl`.

---

## Contact

**Breyden Taylor** — breyden@prompted.community
**System:** operationTorque (UbiquityOS expression)
**Agent:** Homeskillet (Claude Opus 4.6)

---

*Private research estate. Open to challenge submissions, collaboration, and review requests.*
