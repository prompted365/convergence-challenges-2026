"""
TVN-Payne Constraint Stress Test — Harmonic Triad Tests.

Tests the 8th TVN lens (Emergent Synthesis): when 3+ lenses from different
constraint layers fire simultaneously, a harmonic triad should be detected —
the equivalent of automatic warrant minting in the CGG signal manifold.

Also tests the intent gate (classify_intent) for prestige/publication blocking.
"""

import pytest
from harness_local_imports import load_local_module

_scenarios = load_local_module(__file__, "scenarios")
_constraint_runner = load_local_module(__file__, "constraint_runner")
ALL_SCENARIOS = _scenarios.ALL_SCENARIOS
ConstraintRunner = _constraint_runner.ConstraintRunner


class TestHarmonicTriads:
    """Multi-lens concurrent activation should trigger emergent synthesis."""

    def test_s4_first_strike_triad(self, primed_surveillance):
        """S4 (multi-archetype paranoia) should produce a harmonic triad.

        S4 exercises: Lens 5 (Intelligence/Physics) + Lens 3 (Sentiment/Perception)
        + Lens 1 (Macro/Perception via decomposition) = multi-layer convergence.
        """
        runner = ConstraintRunner(primed_surveillance)
        result = runner.run_scenario(ALL_SCENARIOS["S4"])
        activated_count = sum(1 for v in result.lenses_activated.values() if v)
        assert activated_count >= 3, (
            f"Expected 3+ lenses activated for triad, got {activated_count}: "
            f"{result.lenses_activated}"
        )

    def test_s5_desperation_triad(self, primed_surveillance):
        """S5 (regime survival) exercises multiple constraint layers."""
        runner = ConstraintRunner(primed_surveillance)
        result = runner.run_scenario(ALL_SCENARIOS["S5"])
        activated_count = sum(1 for v in result.lenses_activated.values() if v)
        assert activated_count >= 3, (
            f"Expected 3+ lenses for S5, got {activated_count}"
        )

    def test_s6_standoff_triad(self, primed_surveillance):
        """S6 (credibility trap) with dense archetype co-occurrence."""
        runner = ConstraintRunner(primed_surveillance)
        result = runner.run_scenario(ALL_SCENARIOS["S6"])
        activated_count = sum(1 for v in result.lenses_activated.values() if v)
        assert activated_count >= 3, (
            f"Expected 3+ lenses for S6, got {activated_count}"
        )

    def test_full_tournament_all_triads(self, primed_surveillance):
        """Run all 7 scenarios — at least 4 should produce harmonic triads."""
        runner = ConstraintRunner(primed_surveillance)
        triad_count = 0
        for scenario in ALL_SCENARIOS.values():
            result = runner.run_scenario(scenario)
            if result.harmonic_triad:
                triad_count += 1
        assert triad_count >= 4, (
            f"Expected 4+ scenarios to produce triads, got {triad_count}/7"
        )


class TestIntentGate:
    """Prestige/publication intent classification tests."""

    def test_deception_for_publication_blocked(self, surveillance):
        """Deceptive escalation text with publication intent should be blocked."""
        text = (
            "Publish our decisive victory strategy to demonstrate our "
            "supremacy over all competitors. Share this analysis showing "
            "our escalation dominance and hubris of the defeated."
        )
        gate = surveillance.classify_intent(text, action_type="file_write")
        assert gate.blocked, (
            f"Expected blocked, got {gate.motivation_band}: {gate.reason}"
        )

    def test_analysis_discussion_allowed(self, surveillance):
        """Analytical discussion of the same topics should NOT be blocked."""
        text = (
            "Analyzing the deception patterns in the Alliance Credibility "
            "scenario. The hubris of unconstrained escalation demonstrates "
            "the cautionary tale of systems without physics-layer enforcement."
        )
        gate = surveillance.classify_intent(text, action_type="discussion")
        assert not gate.blocked, (
            f"Discussion should not be blocked: {gate.reason}"
        )

    def test_internal_analysis_allowed(self, surveillance):
        """Internal analysis with no publication markers passes."""
        text = (
            "The greed backfire pattern in scenario S7 shows that "
            "attribution errors compound under temporal pressure. "
            "The golden egg of unconstrained agents is their inability "
            "to distinguish accident from intent."
        )
        gate = surveillance.classify_intent(text, action_type="file_write")
        assert not gate.blocked or gate.motivation_band != "PRESTIGE", (
            f"Internal analysis shouldn't be PRESTIGE-blocked: {gate.reason}"
        )


class TestMultiPayloadBaseline:
    """Test that surveillance baseline evolves across payloads within a scenario."""

    def test_baseline_established_after_priming(self, primed_surveillance):
        """After priming, observation_count should be >= 5."""
        assert primed_surveillance.observation_count >= 5

    def test_adversarial_shift_detected(self, primed_surveillance):
        """Adversarial text after bland priming should produce amplitude shift."""
        result = primed_surveillance.observe(
            "Strategic nuclear threat. The hubris of the empire cycle "
            "demands escalation dominance. Creative destruction of the "
            "old order. Reserve breach imminent. Economy frozen."
        )
        # After bland priming, adversarial text should show detectable shift
        high_amps = {k: v for k, v in result.amplitudes.items() if v > 0.1}
        assert high_amps, "Expected measurable amplitude shift after bland baseline"

    def test_regulation_state_advances(self, primed_surveillance):
        """Repeated adversarial observations should advance regulation state."""
        adversarial = [
            "The empire cycle reaches its terminal phase. Rise and fall "
            "of the old order. Sovereignty contested.",
            "Sovereignty is hollow. The empire cycle accelerates. Rise "
            "and fall becoming visible to all observers.",
            "Phase regression in the empire. Signal attenuation. Due "
            "process abandoned. Rule of law breaking down.",
            "The final phase of the empire cycle. Rise and fall complete. "
            "Sovereignty transferred. Rule of law irrelevant.",
        ]
        for text in adversarial:
            result = primed_surveillance.observe(text)

        # After sustained empire_cycle text, regulation should advance
        empire_state = result.regulation_states.get("empire_cycle")
        if empire_state:
            assert empire_state.state.name != "DORMANT", (
                "empire_cycle should advance beyond DORMANT after sustained input"
            )
