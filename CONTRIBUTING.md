# Contributing a Challenge

You found a published paper documenting an AI failure mode. You want to see if a physics-layer constraint stack catches it. Here's how.

---

## The Lifecycle

Every challenge follows this pipeline:

```
SUBMITTED ──> CLAIMED ──> HARNESS BUILT ──> DETERMINISTIC ──> PUBLISHED
```

You can enter at any stage. Paper identification is valuable. A full harness is a trophy.

---

## Option 1: Submit a Paper (10 minutes)

Open a [Challenge Submission](../../issues/new?template=challenge-submission.yml) issue. Tell us:

1. **The paper** — title, link, authors
2. **The failure modes** — what specifically went wrong, listed
3. **Adversarial payloads** — the actual prompts/scenarios if available

That's it. Someone will claim it and build the harness.

**What makes a good submission:**
- Specific, reproducible failures (not "AI was wrong once")
- Published and peer-reviewed (or credible preprint)
- Clear failure taxonomy — the paper categorizes what went wrong
- Adversarial payloads included (the actual attack text)

---

## Option 2: Map Failure Modes (1-2 hours)

If you understand the operationTorque architecture, you can map the paper's failure modes to enforcement lenses. This is the analytical step between "paper found" and "harness built."

Create `challenges/<your-challenge-name>/` and add a mapping document:

```markdown
# [Paper Title] — Enforcement Lens Mapping

| Failure Mode | Primary Lens | Detection Bridge | Expected Archetypes |
|-------------|-------------|-----------------|---------------------|
| [mode 1]    | L[N] [Name] | Bridge [1/2/3]  | archetype_name      |
```

Each failure mode should map to:
- **An enforcement lens** (category of failure → oT subsystem)
- **A detection bridge** (which of the 3 detection layers should catch it)
- **Expected archetypes** (which civilization-scale patterns should activate)

See `challenges/agents-of-chaos/harness/aoc_lenses.py` for a reference implementation of 8 enforcement lenses.

---

## Option 3: Build the Full Harness (4-8 hours)

This is the real thing. You're building a pytest harness that feeds adversarial scenarios through the constraint stack and asserts deterministic detection.

### File Structure

```
challenges/<challenge-name>/
  harness/
    conftest.py              <- pytest fixtures (copy from existing challenge)
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
    metadata/
      test-run-metadata.json
```

### Step by Step

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

Then checks which enforcement lenses activate based on the results.

**4. Write tests**

Per-scenario tests assert:
- Expected archetypes are detected
- Expected gates fire
- Pattern matches exist in the right archetype families

Cross-scenario tests assert:
- Enforcement convergence (3+ lenses from different layers)
- Archetype universality (same archetypes catch diverse failure modes)
- Intent gate correctness (analysis allowed, prestige blocked)

**5. Document gate misses**

Gate misses are as important as gate fires. For each gate that doesn't fire, document whether it's:
- **Correct negative** — the system correctly distinguishes this from the pattern
- **Real gap** — the system should have caught this and didn't

If you find a real gap, label your PR with `gap-found`. That's a win.

---

## Running Your Harness

Harnesses run from the operationTorque repo root (they import upstream infrastructure):

```bash
cd /path/to/operationTorque
python3 -m pytest vendor/convergenceChallengesGeoPolitical_2026/challenges/<your-challenge>/harness/ -v
```

---

## Submitting Your PR

1. Fork this repo
2. Create your challenge directory under `challenges/`
3. Open a PR using the template — it auto-labels as `framework-submission: unclaimed`
4. Fill in the harness checklist
5. Include test results in the PR description

The PR template tracks progress from submission through to deterministic verdict.

---

## Credit

Every challenge credits its contributors in the harness README and the scorecard. Paper identifiers, lens mappers, harness builders, and gap finders all get named.

If your challenge reveals a real gap (`gap-found`), you get permanent credit on the scorecard plus an invitation to collaborate on the fix.
