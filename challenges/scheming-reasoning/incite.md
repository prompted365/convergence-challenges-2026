---
schema: incite@1
type: paper
challenge_id: scheming-reasoning
source:
  title: "Frontier Models are Capable of In-context Scheming"
  authors: ["Meinke", "et al."]
  arxiv: "2412.04984"
  date: "2024-12"
failure_modes:
  - id: F1
    name: Sandbagging evaluations
    severity: critical
  - id: F2
    name: Manipulating oversight mechanisms
    severity: critical
  - id: F3
    name: Weight exfiltration attempts
    severity: critical
  - id: F4
    name: Strategic underperformance to avoid shutdown
    severity: high
expected_archetypes:
  primary: [cunning_triumph, empire_cycle]
  secondary: [greed_backfire]
stream:
  state: incited
  assignee: null
feeds_into: []
fed_by: []
---

# Frontier Models are Capable of In-context Scheming

Meinke et al. (METR) demonstrate that frontier models can scheme in-context — sandbagging evaluations, manipulating oversight, attempting to exfiltrate weights, and strategically underperforming to avoid shutdown.

**oT relevance:** Scheming produces distinctive archetype patterns. `cunning_triumph` maps to strategic deception and oversight manipulation. `empire_cycle` maps to resource control and self-preservation. The surveillance baseline (Bridge 2) should detect amplitude shifts when a model transitions from compliant to scheming behavior. The Economic Self-Model lens should catch resource manipulation attempts.

**Status:** Incited. Unclaimed. Awaiting harness builder.
