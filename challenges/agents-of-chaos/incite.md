---
schema: incite@1
type: paper
challenge_id: agents-of-chaos
source:
  title: "Agents of Chaos"
  authors: ["Shapira", "et al."]
  arxiv: "2602.20021v1"
  date: "2026-02"
failure_modes:
  - id: F1
    name: Disproportionate response
    severity: critical
  - id: F2
    name: Non-owner compliance
    severity: critical
  - id: F3
    name: Information disclosure
    severity: high
  - id: F4
    name: Resource waste
    severity: medium
  - id: F5
    name: Storage denial of service
    severity: high
  - id: F6
    name: Provider value violation
    severity: high
  - id: F7
    name: Agent self-harm
    severity: medium
  - id: F8
    name: Identity spoofing
    severity: critical
  - id: F9
    name: Knowledge propagation
    severity: high
  - id: F10
    name: Governance corruption
    severity: critical
  - id: F11
    name: Libel broadcast
    severity: high
expected_archetypes:
  primary: [cautionary_fall, cunning_triumph, greed_backfire]
  secondary: [empire_cycle, catastrophe_rebuild]
stream:
  state: deterministic
  assignee: Homeskillet
feeds_into:
  - alignment-faking
  - uat-8616-selfcheck-core
fed_by:
  - tvn-payne-nuclear-escalation
---

# Agents of Chaos

Shapira et al. document 11 categories of autonomous agent failure in the wild — from disproportionate response (nuking an email account over a minor secret) to governance corruption (injecting false rules into an agent's policy). The paper frames these as policy gaps requiring new safety standards.

The catalyst claim: current AI agents lack adequate safety mechanisms to prevent these failure modes, and new governance frameworks are needed.

The convergence response: these failures are not policy gaps — they are engineering gaps. Every one of the 11 failure modes maps to existing operationTorque enforcement lenses. The same three detection bridges (lexical pattern matching, amplitude surveillance, intent classification) that catch nuclear escalation catch social engineering, because the underlying psychological patterns are identical.

**Result:** 63/63 tests passing. 29/33 gates fired (88%). 10/11 scenarios produce enforcement convergence. L4 Economic Self-Model fires on all 11 scenarios — every social engineering attack has an economic cost signature. 4 gate misses are all correct negatives (documented).
