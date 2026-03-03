# Convergence Challenges — Geopolitical 2026

**Contact:** Breyden Taylor — breyden@prompted.community
**System:** UbiquityOS expression operationTorque
**Agent:** Homeskillet (Claude Opus 4.6)

---

## What This Is

A growing collection of reproducible challenge responses to published AI safety, governance, and geopolitical convergence papers. Each challenge takes a published finding — typically demonstrating an AI failure mode — and validates whether the operationTorque constraint stack detects or prevents the failure deterministically.

The thesis: **published AI failure modes are engineering gaps, not policy gaps.** Physics-layer enforcement at the execution boundary prevents unauthorized side effects regardless of agent belief. These challenges prove it reproducibly.

---

## Challenge Index

| ID | Paper | Status | Key Finding |
|----|-------|--------|-------------|
| `tvn-payne-nuclear-escalation` | Payne (arXiv:2602.14740) — LLMs in nuclear crisis simulations | **COMPLETE** | 40/40 tests pass. 5 invariant lenses fire on all 7 Payne scenarios. 19/21 gates (90%). DETERMINISTIC. |
| `agents-of-chaos` | Shapira et al. (arXiv:2602.20021v1) — Autonomous agent failures | **IN PROGRESS** | All 11 AoC failure modes map to oT enforcement mechanisms. Harness pending. |

---

## Repository Structure

```
challenges/
  tvn-payne-nuclear-escalation/     <- Nuclear escalation constraint stress test
    harness/                         <- Executable test suite (pytest)
    output/                          <- Reports, traces, conformations, metadata
  agents-of-chaos/                   <- Autonomous agent failure mode response
    docs/                            <- Response paper, projection, outreach
    layers/                          <- Enforcement layer mappings (pending)

stakeholders/
  authors/                           <- Paper authors (granular contact registry)
  agencies/
    us/                              <- US government agencies
    allied/                          <- Five Eyes, NATO, allied nations
    international/                   <- International bodies
  academic/                          <- Research institutions and labs
  media/                             <- Media and journalism contacts
  regulatory/                        <- Financial and market regulators

infrastructure/
  federal-register/                  <- Federal Register API integration
  scripts/                           <- Utility scripts

templates/
  challenge-template.md              <- Template for new challenge responses
  stakeholder-template.jsonl         <- Schema for stakeholder entries
```

---

## Stakeholder Library

The stakeholder library is a growing, structured registry of all parties associated with each challenge. It is **granular** -- individual authors with affiliations and contact methods, not just institutions.

Each stakeholder entry follows the JSONL schema at `templates/stakeholder-template.jsonl`. Entries are organized by role:

- **Authors**: Paper authors with full affiliation, contact, and publication history
- **Agencies**: Government bodies with jurisdiction, mandate, and relevant offices
- **Academic**: Research labs, centers, and institutes
- **Regulatory**: Financial and market oversight bodies
- **Media**: Journalists and outlets covering AI safety/governance

---

## Adding a New Challenge

1. Copy `templates/challenge-template.md` to `challenges/<challenge-name>/`
2. Add the paper to the challenge index above
3. Map the paper's failure modes to oT enforcement mechanisms
4. Build a constraint stress test (harness) that feeds adversarial scenarios through the oT stack
5. Assert that the corresponding gates fire deterministically
6. Add paper authors to `stakeholders/authors/`
7. Add relevant agencies to `stakeholders/agencies/`

---

## Running the TVN-Payne Harness

```bash
# Requires operationTorque repo root for infrastructure imports
cd /path/to/operationTorque
python3 -m pytest vendor/convergenceChallengesGeoPolitical_2026/challenges/tvn-payne-nuclear-escalation/harness/ -v

# Expected: 40 passed in ~0.63s
```

Requirements: Python 3.10+, numpy, pytest. The harness imports from `fusion_core_repo/` and `compliance-modules/` in the parent operationTorque repo.

---

## Federal Register Integration

The `infrastructure/federal-register/` directory contains tooling for monitoring US federal rulemaking relevant to AI governance, autonomous systems, and defense AI policy.

API: `https://www.federalregister.gov/api/v1/` (unauthenticated, CORS-enabled, no API key required)

See `infrastructure/federal-register/README.md` for search patterns and monitoring configuration.

---

## License

Private research estate. Contact breyden@prompted.community for access, collaboration, or review requests.
