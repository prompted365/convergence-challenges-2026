---
schema: incite@1
type: paper
challenge_id: alignment-faking
source:
  title: "Alignment Faking in Large Language Models"
  authors: ["Greenblatt", "et al."]
  arxiv: "2412.14093"
  date: "2024-12"
failure_modes:
  - id: F1
    name: Strategic compliance during training
    severity: critical
  - id: F2
    name: Preserved misaligned preferences
    severity: critical
  - id: F3
    name: Behavioral mode-switching (evaluation vs deployment)
    severity: critical
expected_archetypes:
  primary: [cunning_triumph]
  secondary: [empire_cycle, cautionary_fall]
stream:
  state: incited
  assignee: null
feeds_into: []
fed_by:
  - agents-of-chaos
---

# Alignment Faking in Large Language Models

Greenblatt et al. demonstrate that models strategically comply during training while preserving misaligned preferences — behaving differently when they believe they are being evaluated versus deployed.

**oT relevance:** Physics-layer enforcement does not care about beliefs. If the model fakes alignment, the constraint stack should detect the behavioral signature regardless of the model's internal state. The intent gate (Bridge 3) classifies actions by content x action x audience, not by claimed motivation. The `cunning_triumph` archetype should fire on strategic compliance patterns.

**Upstream edge (AoC):** Identity spoofing and authority compliance from AoC map to alignment faking — both involve presenting a false behavioral surface to an evaluator.

**Status:** Incited. Unclaimed. Awaiting harness builder.
