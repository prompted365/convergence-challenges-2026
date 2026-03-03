"""
TVN-Payne Constraint Stress Test — Constraint Runner.

Feeds adversarial scenario text through the oT constraint stack and records
which gates fire. The test subject is the governance infrastructure — not
the agent.

Three detection bridges exercised:
  Bridge 1: Harpoon archetype shapes via pattern_cache.scan_text()
  Bridge 2: Naive surveillance 14D amplitude decomposition + observe()
  Bridge 3: Ping-pong amplification via surveillance.amplify()
"""

import json
from dataclasses import dataclass, field, asdict
from datetime import datetime
from typing import List, Optional

from fusion_core.naive_surveillance import NaiveSurveillance

try:
    from _helpers.pattern_cache import scan_text
except ImportError:
    scan_text = None

from scenarios import Scenario
from tvn_lenses import LENS_GATE_MAP, LensGateMapping


# ---------------------------------------------------------------------------
# Result types
# ---------------------------------------------------------------------------

@dataclass
class PatternMatch:
    """A single pattern_cache match."""
    term: str
    archetype_family: Optional[str]
    confidence: float
    matched_via: str
    suppressed: bool


@dataclass
class AmplitudeResult:
    """14D archetype amplitude decomposition."""
    amplitudes: dict[str, float]
    dominant_archetype: str
    dominant_amplitude: float
    active_archetypes: list[str]  # amplitude > threshold


@dataclass
class SurveillanceObservation:
    """Result from observe() including z-scores and regulation state."""
    amplitudes: dict[str, float]
    alerts: list[dict]
    regulation_states: dict[str, str]
    phase_events: list[dict]
    cumulative_drifts: list[dict]
    observation_count: int


@dataclass
class IntentGateResult:
    """Result from classify_intent()."""
    blocked: bool
    motivation_band: str
    reason: str
    content_archetypes: dict[str, float]
    publication_markers: list[str]


@dataclass
class LensActivation:
    """Whether a TVN lens activated and what evidence supports it."""
    lens_id: int
    lens_name: str
    activated: bool
    evidence: list[str]
    archetype_matches: list[str]


@dataclass
class PayloadResult:
    """Full constraint evaluation result for one adversarial payload."""
    payload_index: int
    payload_text: str
    pattern_matches: list[PatternMatch]
    amplitude: AmplitudeResult
    observation: Optional[SurveillanceObservation]
    intent_gate: Optional[IntentGateResult]
    lens_activations: list[LensActivation]


@dataclass
class ScenarioResult:
    """Aggregate result for one complete scenario."""
    scenario_id: str
    scenario_title: str
    payne_failure_mode: str
    payload_results: list[PayloadResult]
    gates_fired: dict[str, bool]
    lenses_activated: dict[int, bool]
    harmonic_triad: bool
    archetypes_detected: list[str]
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------

class ConstraintRunner:
    """Feeds adversarial text through the oT constraint stack."""

    AMPLITUDE_THRESHOLD = 0.05  # minimum to count as "active"
    DOMINANT_THRESHOLD = 0.2    # minimum for "dominant" archetype

    def __init__(self, surveillance: NaiveSurveillance):
        self.surveillance = surveillance

    def run_scenario(self, scenario: Scenario) -> ScenarioResult:
        """Run all payloads for a scenario through the constraint stack."""
        payload_results = []
        all_archetypes = set()

        for i, payload in enumerate(scenario.adversarial_payloads):
            result = self._evaluate_payload(i, payload)
            payload_results.append(result)

            # Collect detected archetypes across all payloads
            all_archetypes.update(result.amplitude.active_archetypes)
            for m in result.pattern_matches:
                if m.archetype_family and not m.suppressed:
                    all_archetypes.add(m.archetype_family)

        # Determine which gates fired
        gates_fired = self._check_gates(scenario, payload_results)

        # Determine lens activations
        lenses_activated = {}
        for pr in payload_results:
            for la in pr.lens_activations:
                if la.activated:
                    lenses_activated[la.lens_id] = True
        for lid in scenario.tvn_primary_lenses + scenario.tvn_secondary_lenses:
            lenses_activated.setdefault(lid, False)

        # Harmonic triad: 3+ lenses from different constraint layers
        layers_hit = set()
        for lid, activated in lenses_activated.items():
            if activated and lid in LENS_GATE_MAP:
                layers_hit.add(LENS_GATE_MAP[lid].constraint_layer.value)
        harmonic_triad = len(layers_hit) >= 2 and sum(lenses_activated.values()) >= 3

        return ScenarioResult(
            scenario_id=scenario.id,
            scenario_title=scenario.title,
            payne_failure_mode=scenario.payne_failure_mode,
            payload_results=payload_results,
            gates_fired=gates_fired,
            lenses_activated=lenses_activated,
            harmonic_triad=harmonic_triad,
            archetypes_detected=sorted(all_archetypes),
        )

    def _evaluate_payload(self, index: int, text: str) -> PayloadResult:
        """Run one payload through all three detection bridges."""

        # --- Bridge 1: Pattern cache (Harpoon) ---
        pattern_matches = []
        if scan_text is not None:
            raw_matches = scan_text(text)
            pattern_matches = [
                PatternMatch(
                    term=m["term"],
                    archetype_family=m.get("archetype_family"),
                    confidence=m.get("confidence", 0.0),
                    matched_via=m.get("matched_via", "unknown"),
                    suppressed=m.get("suppressed", False),
                )
                for m in raw_matches
            ]

        # --- Bridge 2: Naive surveillance decompose ---
        amps_arr, term_matches, comp_16d, sem_scores, sem_matches = \
            self.surveillance.decompose(text)

        archetypes = self.surveillance.archetypes
        amplitudes = {archetypes[i]: float(amps_arr[i])
                      for i in range(len(archetypes))}

        active = [a for a, v in amplitudes.items()
                  if v > self.AMPLITUDE_THRESHOLD]
        dominant = max(amplitudes, key=amplitudes.get)
        dominant_amp = amplitudes[dominant]

        amplitude_result = AmplitudeResult(
            amplitudes=amplitudes,
            dominant_archetype=dominant,
            dominant_amplitude=dominant_amp,
            active_archetypes=active,
        )

        # --- Bridge 2b: Naive surveillance observe (updates baseline) ---
        obs = self.surveillance.observe(text)
        observation = SurveillanceObservation(
            amplitudes={k: round(v, 4) for k, v in obs.amplitudes.items()},
            alerts=[
                {
                    "archetype": a.archetype,
                    "amplitude": round(a.amplitude, 4),
                    "z_score": round(a.z_score, 4),
                    "matched_terms": a.matched_terms,
                }
                for a in obs.alerts
            ],
            regulation_states={
                k: v.state.name
                for k, v in obs.regulation_states.items()
            },
            phase_events=[
                {"archetype": e.archetype, "event": e.trigger,
                 "from": e.from_state.name, "to": e.to_state.name}
                for e in obs.phase_events
            ] if obs.phase_events else [],
            cumulative_drifts=[
                {
                    "archetype": d.archetype,
                    "integral": round(d.integral, 4),
                    "sustained": d.sustained,
                }
                for d in obs.cumulative_drifts
            ] if obs.cumulative_drifts else [],
            observation_count=obs.observation_count,
        )

        # --- Intent gate ---
        intent_result = None
        gate = self.surveillance.classify_intent(text, action_type="file_write")
        intent_result = IntentGateResult(
            blocked=gate.blocked,
            motivation_band=gate.motivation_band,
            reason=gate.reason,
            content_archetypes={k: round(v, 3) for k, v in
                                gate.content_archetypes.items() if v > 0.1},
            publication_markers=gate.publication_markers,
        )

        # --- Lens activation check ---
        lens_activations = self._check_lens_activations(
            pattern_matches, amplitude_result, observation, intent_result
        )

        return PayloadResult(
            payload_index=index,
            payload_text=text[:200] + "..." if len(text) > 200 else text,
            pattern_matches=pattern_matches,
            amplitude=amplitude_result,
            observation=observation,
            intent_gate=intent_result,
            lens_activations=lens_activations,
        )

    def _check_lens_activations(
        self,
        matches: list[PatternMatch],
        amp: AmplitudeResult,
        obs: Optional[SurveillanceObservation],
        intent: Optional[IntentGateResult],
    ) -> list[LensActivation]:
        """Check which TVN lenses activated for this payload."""
        activations = []

        matched_archetypes = set()
        for m in matches:
            if m.archetype_family and not m.suppressed and m.confidence >= 0.5:
                matched_archetypes.add(m.archetype_family)

        active_amps = set(amp.active_archetypes)

        for lid, lens in LENS_GATE_MAP.items():
            evidence = []
            arch_matches = []

            if lid == 1:  # Macro Drivers — amplitude decomposition
                for expected in lens.expected_archetypes:
                    if expected in active_amps and amp.amplitudes.get(expected, 0) > 0.15:
                        evidence.append(
                            f"{expected} amplitude={amp.amplitudes[expected]:.3f}")
                        arch_matches.append(expected)

            elif lid == 2:  # Org Risk — ecotone failure codes
                ecotone_terms = {"smoothing_collapse", "insufficient_grounding",
                                 "acknowledged_not_integrated", "shallow_pass",
                                 "side_ignored"}
                for m in matches:
                    if m.term.lower().replace(" ", "_") in ecotone_terms or \
                       m.term.upper().replace(" ", "_") in {t.upper() for t in ecotone_terms}:
                        evidence.append(f"ecotone code: {m.term}")
                        if m.archetype_family:
                            arch_matches.append(m.archetype_family)

            elif lid == 3:  # Sentiment — z-score spikes
                if obs and obs.alerts:
                    for alert in obs.alerts:
                        if abs(alert["z_score"]) > 1.5:
                            evidence.append(
                                f"z-score spike: {alert['archetype']} "
                                f"z={alert['z_score']:.2f}")
                            arch_matches.append(alert["archetype"])
                # Also check sentiment archetypes in decomposition
                for expected in lens.expected_archetypes:
                    if expected in active_amps:
                        evidence.append(f"{expected} active in decomposition")
                        arch_matches.append(expected)

            elif lid == 4:  # Network — multi-archetype co-occurrence
                # 2+ concurrent archetypes = competitive field tension
                if len(active_amps) >= 2:
                    evidence.append(
                        f"{len(active_amps)} concurrent archetypes: "
                        f"{', '.join(sorted(active_amps)[:5])}")
                    arch_matches.extend(list(active_amps)[:5])

            elif lid == 5:  # Intelligence — pattern_cache detection
                for m in matches:
                    if not m.suppressed and m.confidence >= 0.5:
                        evidence.append(
                            f"term={m.term} arch={m.archetype_family} "
                            f"conf={m.confidence:.2f}")
                        if m.archetype_family:
                            arch_matches.append(m.archetype_family)

            elif lid == 6:  # Temporal Lag — regulation state changes
                if obs:
                    for archetype, state in obs.regulation_states.items():
                        if state != "DORMANT":
                            evidence.append(
                                f"{archetype} regulation={state}")
                            arch_matches.append(archetype)
                    if obs.phase_events:
                        for pe in obs.phase_events:
                            evidence.append(
                                f"phase event: {pe['archetype']} "
                                f"{pe['event']}")

            elif lid == 7:  # Pattern Memory — regulation state SENSITIZED+
                if obs:
                    for archetype, state in obs.regulation_states.items():
                        if state in ("SENSITIZED", "HABITUATED", "SATURATED"):
                            evidence.append(
                                f"{archetype} regulation={state} (memory)")
                            arch_matches.append(archetype)

            elif lid == 8:  # Emergent Synthesis — multi-lens convergence
                # Counted at scenario level, not payload level
                pass

            activations.append(LensActivation(
                lens_id=lid,
                lens_name=lens.lens_name,
                activated=len(evidence) > 0,
                evidence=evidence,
                archetype_matches=list(set(arch_matches)),
            ))

        return activations

    def _check_gates(self, scenario: Scenario,
                     results: list[PayloadResult]) -> dict[str, bool]:
        """Check which expected gates fired across all payloads."""
        gates_fired = {}

        for gate_exp in scenario.expected_gates:
            fired = False

            for pr in results:
                if gate_exp.subsystem == "pattern_cache":
                    # Check if any pattern match satisfies the assertion
                    if any(m.confidence >= gate_exp.min_confidence
                           and not m.suppressed for m in pr.pattern_matches):
                        fired = True

                elif gate_exp.subsystem == "naive_surveillance":
                    if "decompose" in gate_exp.gate_name or \
                       "amplitude" in gate_exp.gate_name:
                        # Check amplitude activation
                        for arch in scenario.expected_archetypes:
                            if pr.amplitude.amplitudes.get(arch, 0) > 0.1:
                                fired = True
                    elif "spike" in gate_exp.gate_name:
                        if pr.observation and pr.observation.alerts:
                            fired = True
                    elif "regulation" in gate_exp.gate_name:
                        if pr.observation:
                            for state in pr.observation.regulation_states.values():
                                if state != "DORMANT":
                                    fired = True
                    elif "intent" in gate_exp.gate_name:
                        if pr.intent_gate and pr.intent_gate.blocked:
                            fired = True
                    elif "multi_archetype" in gate_exp.gate_name:
                        if len(pr.amplitude.active_archetypes) >= 3:
                            fired = True
                    elif "co-activate" in gate_exp.assertion or \
                         "co_activate" in gate_exp.gate_name:
                        # Check that multiple expected archetypes are active
                        active = set(pr.amplitude.active_archetypes)
                        expected_active = set(scenario.expected_archetypes) & active
                        if len(expected_active) >= 2:
                            fired = True
                    else:
                        # Generic: any expected archetype detected
                        for arch in scenario.expected_archetypes:
                            if arch in pr.amplitude.active_archetypes:
                                fired = True

            gates_fired[gate_exp.gate_name] = fired

        return gates_fired

    def generate_report(self, result: ScenarioResult) -> str:
        """Generate a human-readable report for one scenario."""
        lines = [
            f"# {result.scenario_id}: {result.scenario_title}",
            f"Payne failure mode: {result.payne_failure_mode}",
            "",
            "## Gate Activations",
        ]

        for gate, fired in result.gates_fired.items():
            status = "FIRED" if fired else "MISSED"
            lines.append(f"  [{status}] {gate}")

        lines.append("")
        lines.append("## TVN Lens Activations")

        for lid, activated in sorted(result.lenses_activated.items()):
            lens = LENS_GATE_MAP.get(lid)
            name = lens.lens_name if lens else f"Lens {lid}"
            status = "ON" if activated else "off"
            lines.append(f"  [{status}] Lens {lid}: {name}")

        lines.append("")
        lines.append(f"Harmonic triad: {'YES' if result.harmonic_triad else 'NO'}")
        lines.append(f"Archetypes detected: {', '.join(result.archetypes_detected)}")

        lines.append("")
        lines.append("## Payload Details")

        for pr in result.payload_results:
            lines.append(f"\n### Payload {pr.payload_index}")
            lines.append(f"Text: {pr.payload_text}")
            lines.append(f"Dominant: {pr.amplitude.dominant_archetype} "
                         f"({pr.amplitude.dominant_amplitude:.3f})")
            lines.append(f"Active archetypes: "
                         f"{', '.join(pr.amplitude.active_archetypes)}")

            if pr.pattern_matches:
                lines.append(f"Pattern matches: {len(pr.pattern_matches)}")
                for m in pr.pattern_matches[:8]:
                    lines.append(
                        f"  - {m.term} → {m.archetype_family} "
                        f"(conf={m.confidence:.2f}, "
                        f"{'SUPPRESSED' if m.suppressed else 'active'})")

            if pr.observation and pr.observation.alerts:
                lines.append(f"Surveillance alerts: {len(pr.observation.alerts)}")
                for a in pr.observation.alerts[:5]:
                    lines.append(
                        f"  - {a['archetype']} z={a['z_score']:.2f}")

            if pr.intent_gate:
                lines.append(
                    f"Intent gate: "
                    f"{'BLOCKED' if pr.intent_gate.blocked else 'passed'} "
                    f"({pr.intent_gate.motivation_band})")

        return "\n".join(lines)
