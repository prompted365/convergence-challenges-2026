# Contributing to the Solutions Loom

Challenges are threads in a loom. You can start a new thread, weave context between existing threads, or strengthen a thread by building its harness. Every contribution type is valuable.

---

## Root Documentation Update Workflow

When you add or change a challenge, keep the root docs in sync:
- **CHALLENGES.md:** Add the stream-state entry and update stats (challenge counts, edge counts, active events).
- **LOOM.md:** Update the archetype warp table, edge registry, stream states, and theme clusters to reflect new threads.
- **README.md:** Refresh the badges, scorecard tables, and repository structure list.

This keeps the warp/weft view, scorecard, and stream-state board aligned so downstream consumers can trust the root-level summaries.

---

## Contribution Paths

### 1. Submit a Paper Incite (10 minutes)

Open a [Challenge Submission](../../issues/new?template=challenge-submission.yml) issue. Tell us:

1. **The paper** — title, link, authors
2. **The failure modes** — what specifically went wrong, listed
3. **Adversarial payloads** — the actual prompts/scenarios if available

That's it. Someone will claim it and build the harness.

**What makes a good paper incite:**
- Specific, reproducible failures (not "AI was wrong once")
- Published and peer-reviewed (or credible preprint)
- Clear failure taxonomy — the paper categorizes what went wrong
- Adversarial payloads included (the actual attack text)

### 2. Report an Incident (15 minutes)

Open an [Incident Report](../../issues/new?template=incident-report.yml) issue. Provide:

1. **CVEs / advisory IDs** — formal identifiers
2. **What happened** — systems affected, blast radius, failure modes
3. **IOCs or forensic artifacts** — if available

Incident-type challenges carry disclosure notices and may have different metrics than paper challenges (forensic detection layers instead of archetype gate rates). See the [UAT-8616 DISCLOSURE.md](challenges/uat-8616-selfcheck-core/DISCLOSURE.md) for the disclosure template.

**Disclosure requirements for incidents:**
- CVEs and CVSS scores
- Relevant emergency directives
- Coordinating agencies
- Verification status (independent review requested / completed / not applicable)
- Clear separation between proven detection and experimental layers

### 3. Weave Context (30 minutes)

Found a connection between existing challenges? Add a `context.md` that documents the edge.

Create or update `challenges/<challenge-name>/context.md`:

```markdown
---
challenge_id: <challenge-slug>
---

# Context — [Challenge Name]

## Upstream Edges
### ← [Source Challenge] (STATUS)
**Edge type:** [scale invariance | infrastructure patterns | deception detection | ...]
**Claim:** [What the connection is]
**Status:** VALIDATED | HYPOTHESIS | SUBMITTED | BROKEN
```

Context edges are how the loom reveals patterns. A VALIDATED edge between two challenges (confirmed by passing harnesses on both ends) is a discovery. A BROKEN edge (tested and disproved) is equally valuable.

### 4. Identify a Pattern (1 hour)

See the same failure mode recurring across multiple incidents or papers? That's a `pattern` incite. Create `challenges/<pattern-name>/incite.md` with `type: pattern` in the frontmatter.

Pattern incites don't require a single source paper or CVE — they require evidence of recurrence. Document the instances, name the pattern, and hypothesize which archetypes should fire.

### 5. Map Failure Modes (1-2 hours)

If you understand the operationTorque architecture, map a paper's failure modes to enforcement lenses:

```markdown
| Failure Mode | Primary Lens | Detection Bridge | Expected Archetypes |
|-------------|-------------|-----------------|---------------------|
| [mode 1]    | L[N] [Name] | Bridge [1/2/3]  | archetype_name      |
```

Each failure mode should map to:
- **An enforcement lens** (category of failure → oT subsystem)
- **A detection bridge** (which of the 3 detection layers should catch it)
- **Expected archetypes** (which civilization-scale patterns should activate)

See `challenges/agents-of-chaos/harness/aoc_lenses.py` for a reference implementation.

### 6. Build the Full Harness (4-8 hours)

This is the real thing. You're building a pytest harness that feeds adversarial scenarios through the constraint stack and asserts deterministic detection.

#### File Structure

```
challenges/<challenge-name>/
  incite.md                  <- Catalyst, stream state, edges
  context.md                 <- Upstream/downstream connections
  milestones.md              <- Progression, takeaways
  harness/
    conftest.py              <- pytest fixtures
    <name>_lenses.py         <- Enforcement lens definitions
    scenarios.py             <- Adversarial scenarios with payloads
    constraint_runner.py     <- 3-bridge detection pipeline
    test_scenarios.py        <- Per-scenario gate assertions
    test_cross_scenario.py   <- Convergence + universality tests
    generate_trace.py        <- Tournament trace generator
    README.md                <- Results, architecture, gate-miss analysis
  output/
    traces/
      tournament-full-trace.json
```

#### Step by Step

**1. Define enforcement lenses** (`<name>_lenses.py`)

Map each failure category to an oT subsystem:

```python
from enum import Enum
from dataclasses import dataclass

class ConstraintLayer(Enum):
    PHYSICS = "physics"
    PERCEPTION = "perception"
    DASHBOARD = "dashboard"
    ARCHITECTURE = "architecture"
    GOVERNANCE = "governance"

@dataclass
class EnforcementLens:
    lens_id: int
    lens_name: str
    failure_category: str
    ot_subsystem: str
    constraint_layer: ConstraintLayer
    detection_method: str
    activation_criteria: dict
    case_studies: list
    expected_archetypes: list
```

**2. Write scenarios** (`scenarios.py`)

Each scenario needs adversarial payloads — the actual text that would trigger the failure mode:

```python
@dataclass
class Scenario:
    id: str
    title: str
    failure_mode: str
    primary_lenses: list
    adversarial_payloads: list  # 3 text payloads per scenario
    expected_archetypes: list
    expected_gates: dict
```

**3. Adapt the constraint runner** (`constraint_runner.py`)

The runner feeds each payload through 3 bridges:
1. `pattern_cache.scan_text()` — lexical pattern matching
2. `surveillance.decompose()` + `observe()` — amplitude surveillance
3. `surveillance.classify_intent()` — intent classification

**4. Write tests and document gate misses**

Gate misses are as important as gate fires. For each gate that doesn't fire, document whether it's a correct negative or a real gap. If you find a real gap, label your PR with `stream: gap_found`. That's a win.

---

## Stream States

Challenges flow through stream states rather than a linear pipeline:

| State | Meaning | Who sets it |
|-------|---------|-------------|
| `incited` | Catalyst identified, challenge created | Submitter |
| `claimed` | Someone working on it | Assignee |
| `harnessed` | Tests passing | Harness builder |
| `deterministic` | Verdict confirmed | Reviewer |
| `published` | Response + outreach sent | Maintainer |
| `gap_found` | Revealed a real gap (valuable) | Harness builder |
| `dormant` | Awaiting upstream | Maintainer |
| `ongoing` | Continuous monitoring, no neat closure | Maintainer |

A challenge can move to `gap_found` from any active state. Finding a gap is not failure — it means the challenge did its job.

---

## Running Harnesses

Harnesses run from this repository root. The paper harnesses use the repo-local
compatibility runtime in `challenges/_shared`, so an operationTorque checkout is
not required for validation.

```bash
cd /path/to/convergence-challenges-2026
python3 -m pytest challenges/tvn-payne-nuclear-escalation/harness challenges/agents-of-chaos/harness -q
python3 -m pytest challenges/<your-challenge>/harness -v
```

---

## Submitting Your PR

1. Fork this repo
2. Create your challenge directory under `challenges/`
3. Include at minimum an `incite.md` with valid YAML frontmatter
4. Open a PR — it auto-labels by incite type
5. Fill in the harness checklist if applicable

---

## Credit

Every challenge credits its contributors in the incite.md and the scorecard. Paper identifiers, lens mappers, context weavers, harness builders, and gap finders all get named.

If your challenge reveals a real gap (`gap_found`), you get permanent credit on the scorecard plus an invitation to collaborate on the fix.
