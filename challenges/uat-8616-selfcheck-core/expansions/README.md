# Expansion Layers — selfcheck-core

These are experimental detection theories initiated from the base kit's forensic detection surface. They explore whether deeper invariant-based detection is possible beyond known IOC matching.

**These layers are not part of the current independent review request.** They represent brainstorming, theory, and early implementation — not proven, production-applicable detection.

## Layers

| Layer | Module | Approach | Maturity |
|-------|--------|----------|----------|
| 6 | `geom.rs` | Three-axis geometric drift scoring with quadrant classification | Experimental |
| 7 | `sonar.rs` | Three-band echo detection at acoustic/optical/gravitational timescales | Experimental |
| 8 | `tripwire.rs` | Probabilistic tripwires + McFly lineage verification for timeline inconsistency | Experimental |
| 9 | `eris.rs` | Cross-domain narrative coherence (filesystem/process/cache/log/entropy) | Experimental |
| 10 | `spinaltap.rs` | Criticality inversion scoring + invariant lock violations + Shannon entropy | Experimental |
| 11 | `redshift.rs` | Adversarial robustness validation against 18 simulated evasion vectors | Experimental |
| 12 | `anchor.rs` | Physics-anchored invariant proofs (thermodynamic, causal, observational) | Experimental |

## Relationship to Base Kit

The base kit (layers 0–5b) detects compromise using known indicators. These expansion layers ask: *what can you detect when the attacker has erased every known indicator?*

Each expansion layer was initiated from a real detection gap identified during base kit development. They are organized here — rather than in the base kit's review scope — because their theoretical foundations have not been independently validated.

## Documents

- **STRATEGIC_FRAMEWORK.md** — TVN leverage framework mapping financial volatility detection to forensic detection. The theoretical bridge between operationTorque's constraint stack and infrastructure forensics.

## Promotion Path

Individual expansion layers may be promoted to review scope as they mature. Promotion criteria:
1. Theoretical basis documented and peer-reviewable
2. False positive rate characterized on clean appliance baselines
3. Detection gap it addresses is confirmed by independent incident analysis

Expansion maturation does not gate the base kit's review process.
