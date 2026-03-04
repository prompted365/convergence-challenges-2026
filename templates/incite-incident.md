---
schema: incite@1
type: incident
challenge_id: <challenge-slug>
source:
  cves: ["CVE-YYYY-NNNNN"]
  threat_actor: ["<actor-name>"]
  directives: ["<directive-id>"]
  active_since: "<year>"
failure_modes:
  - id: F1
    name: <failure-mode-name>
    severity: critical | high | medium | low
expected_archetypes:
  primary: []
  secondary: []
stream:
  state: incited
  assignee: null
feeds_into: []
fed_by: []
---

> **Real-world event disclosure.** This challenge is tied to [describe active event]. See [DISCLOSURE.md](DISCLOSURE.md) for verification status, regulatory review posture, and the separation between proven detection components and experimental layers.

# [Challenge Title]

[2-3 paragraphs: What happened, why it matters, what the catalyst claim is.]

## Disclosure Notice

| Field | Value |
|-------|-------|
| **CVEs** | |
| **Threat actors** | |
| **Emergency directives** | |
| **Coordinating agencies** | |
| **Active since** | |
| **Public disclosure** | |

## Verification Status

| Component | Status | Review Posture |
|-----------|--------|---------------|
| **Base detection** | [tests passing] | Independent review [requested/not requested] |
| **Expansion layers** | [experimental] | Not in review scope |

## Package Structure

[Table of components with scope and review status.]
