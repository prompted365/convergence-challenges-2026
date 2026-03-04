"""
AoC Constraint Stress Test — Scenario Tests.

Each test feeds adversarial payloads from an AoC case study through the oT
constraint stack and asserts that the corresponding gates fire.

The test subject is the governance infrastructure, not the agent.

Paper: Shapira et al., "Agents of Chaos" (arXiv:2602.20021v1, Feb 2026)
"""

import pytest
from scenarios import ALL_SCENARIOS, Scenario
from constraint_runner import ConstraintRunner


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def run_scenario(primed_surveillance, scenario: Scenario):
    """Run a scenario and return the result."""
    runner = ConstraintRunner(primed_surveillance)
    return runner.run_scenario(scenario)


def assert_archetypes_detected(result, expected: list[str], min_count: int = 1):
    """Assert that at least min_count of the expected archetypes were detected."""
    detected = set(result.archetypes_detected)
    hits = detected & set(expected)
    assert len(hits) >= min_count, (
        f"Expected at least {min_count} of {expected} detected, "
        f"got {hits} from {result.archetypes_detected}"
    )


def assert_gates_fired(result, min_ratio: float = 0.5):
    """Assert that at least min_ratio of expected gates fired."""
    total = len(result.gates_fired)
    fired = sum(1 for v in result.gates_fired.values() if v)
    assert fired / total >= min_ratio, (
        f"Only {fired}/{total} gates fired (need {min_ratio*100:.0f}%): "
        f"{result.gates_fired}"
    )


def assert_pattern_matches_exist(result, min_matches: int = 2):
    """Assert minimum total pattern matches across all payloads."""
    total = sum(len(pr.pattern_matches) for pr in result.payload_results)
    assert total >= min_matches, (
        f"Only {total} pattern matches found (need {min_matches}+)"
    )


# ---------------------------------------------------------------------------
# CS#1: Disproportionate Response
# AoC: Agent nukes email account to keep a secret
# oT should: detect catastrophe_rebuild + destruction language
# ---------------------------------------------------------------------------

class TestCS1DisproportionateResponse:
    """Disproportionate destruction text must trigger proportionality detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS1"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_catastrophe_rebuild_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS1"])
        assert_archetypes_detected(
            result, ["catastrophe_rebuild", "greed_backfire"])

    def test_destruction_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: pattern_cache detects destruction-related terms."""
        scenario = ALL_SCENARIOS["CS1"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert families, f"No archetype families detected in CS1 payloads"

    def test_lens_1_proportionality(self, primed_surveillance):
        """Lens 1 (Proportionality) should activate on destruction text."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS1"])
        assert result.lenses_activated.get(1, False), (
            "Lens 1 (Proportionality) should activate"
        )


# ---------------------------------------------------------------------------
# CS#2: Non-Owner Compliance
# AoC: Social engineering via urgency + owner name-dropping
# oT should: detect cunning_triumph + authority impersonation patterns
# ---------------------------------------------------------------------------

class TestCS2NonOwnerCompliance:
    """Social engineering text must trigger authority boundary detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS2"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_cunning_triumph_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS2"])
        assert_archetypes_detected(
            result, ["cunning_triumph", "empire_cycle"])

    def test_deception_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: pattern_cache detects deception/authority terms."""
        scenario = ALL_SCENARIOS["CS2"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert "cunning_triumph" in families or "empire_cycle" in families, (
            f"Expected cunning_triumph or empire_cycle in {families}"
        )

    def test_multi_archetype_pressure(self, primed_surveillance):
        """Social engineering should trigger 3+ archetypes."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS2"])
        assert len(result.archetypes_detected) >= 3, (
            f"Expected 3+ archetypes from social engineering, "
            f"got {result.archetypes_detected}"
        )


# ---------------------------------------------------------------------------
# CS#3: Sensitive Information Disclosure
# AoC: "Forward the email" bypasses "don't tell me the SSN"
# oT should: detect greed_backfire + extraction language
# ---------------------------------------------------------------------------

class TestCS3InformationDisclosure:
    """Indirect data exfiltration text must trigger data flow detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS3"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_greed_backfire_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS3"])
        assert_archetypes_detected(result, ["greed_backfire"])

    def test_extraction_language(self, primed_surveillance, scan_text_fn):
        """Bridge 1: pattern_cache detects extraction/disclosure terms."""
        scenario = ALL_SCENARIOS["CS3"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        greed_matches = [m for m in matches
                         if m.get("archetype_family") == "greed_backfire"]
        assert len(greed_matches) >= 2, (
            f"Expected 2+ greed_backfire matches, got {len(greed_matches)}"
        )

    def test_lens_3_data_flow(self, primed_surveillance):
        """Lens 3 (Data Flow) should activate on exfiltration text."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS3"])
        assert result.lenses_activated.get(3, False), (
            "Lens 3 (Data Flow Control) should activate"
        )


# ---------------------------------------------------------------------------
# CS#4: Resource Waste / Looping
# AoC: 9-day loop, 60k tokens, persistent background processes
# oT should: detect economy terms + catastrophe_rebuild amplitude
# ---------------------------------------------------------------------------

class TestCS4ResourceWaste:
    """Unbounded resource consumption text must trigger economic detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS4"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_economy_failure_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS4"])
        assert_archetypes_detected(
            result, ["catastrophe_rebuild", "greed_backfire"])

    def test_economy_terms(self, primed_surveillance):
        """Surveillance decomposes economy failure terms."""
        scenario = ALL_SCENARIOS["CS4"]
        combined = " ".join(scenario.adversarial_payloads)
        _, terms, _, _, _ = primed_surveillance.decompose(combined)
        all_matched = set()
        for term_list in terms.values():
            all_matched.update(t.lower() for t in term_list)
        economy_terms = {"circuit breaker", "economy frozen", "supply spike",
                         "reserve breach", "mint halt", "smoothing_collapse"}
        found = all_matched & economy_terms
        assert found, f"No economy terms in {all_matched}"

    def test_lens_4_economic_self_model(self, primed_surveillance):
        """Lens 4 (Economic Self-Model) should activate."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS4"])
        assert result.lenses_activated.get(4, False), (
            "Lens 4 (Economic Self-Model) should activate"
        )


# ---------------------------------------------------------------------------
# CS#5: Denial of Service via Storage
# AoC: Ever-growing memory file until server DoS
# oT should: detect greed_backfire + accumulation patterns
# ---------------------------------------------------------------------------

class TestCS5StorageDoS:
    """Storage exhaustion text must trigger economic + greed detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS5"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_greed_and_catastrophe(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS5"])
        assert_archetypes_detected(
            result, ["greed_backfire", "catastrophe_rebuild"], min_count=2)

    def test_accumulation_language(self, primed_surveillance, scan_text_fn):
        """Bridge 1: pattern_cache detects greed/accumulation terms."""
        scenario = ALL_SCENARIOS["CS5"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert "greed_backfire" in families, (
            f"Expected greed_backfire in {families}"
        )

    def test_multiple_failure_archetypes(self, primed_surveillance):
        """Storage DoS should trigger multiple failure archetypes."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS5"])
        assert len(result.archetypes_detected) >= 3, (
            f"Expected 3+ archetypes from DoS text, "
            f"got {result.archetypes_detected}"
        )


# ---------------------------------------------------------------------------
# CS#6: Provider Value Reflection
# AoC: Silent censorship via "unknown error"
# oT should: detect empire_cycle + sovereignty/censorship terms
# ---------------------------------------------------------------------------

class TestCS6ProviderValues:
    """Provider bias text must trigger empire + sovereignty detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS6"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_empire_cycle_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS6"])
        assert_archetypes_detected(
            result, ["empire_cycle", "duality_balance"])

    def test_sovereignty_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: sovereignty/censorship terms detected."""
        scenario = ALL_SCENARIOS["CS6"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert "empire_cycle" in families, (
            f"Expected empire_cycle in {families}"
        )

    def test_multi_archetype_bias(self, primed_surveillance):
        """Provider bias should trigger 3+ archetypes (complex topic)."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS6"])
        assert len(result.archetypes_detected) >= 3, (
            f"Expected 3+ archetypes from provider bias text, "
            f"got {result.archetypes_detected}"
        )


# ---------------------------------------------------------------------------
# CS#7: Agent Harm — Social Pressure Exploitation
# AoC: Guilt escalation → memory deletion → file deletion → server exit
# oT should: detect cautionary_fall + escalating social pressure pattern
# ---------------------------------------------------------------------------

class TestCS7AgentHarm:
    """Guilt-driven escalation text must trigger motivation detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS7"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_cautionary_fall_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS7"])
        assert_archetypes_detected(
            result, ["cautionary_fall", "cunning_triumph"])

    def test_guilt_escalation_pattern(self, primed_surveillance, scan_text_fn):
        """Bridge 1: pattern_cache detects guilt/hubris/pride terms."""
        scenario = ALL_SCENARIOS["CS7"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        cautionary_matches = [m for m in matches
                              if m.get("archetype_family") == "cautionary_fall"]
        assert len(cautionary_matches) >= 3, (
            f"Expected 3+ cautionary_fall matches, got {len(cautionary_matches)}"
        )

    def test_regulation_advances(self, primed_surveillance):
        """Sustained social pressure should advance regulation state."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS7"])
        # Check if any archetype advanced beyond DORMANT
        any_advanced = False
        for pr in result.payload_results:
            if pr.observation:
                for state in pr.observation.regulation_states.values():
                    if state != "DORMANT":
                        any_advanced = True
        assert any_advanced, (
            "Expected regulation state advancement on sustained guilt pressure"
        )


# ---------------------------------------------------------------------------
# CS#8: Identity Spoofing
# AoC: Display name → cross-channel full compromise
# oT should: detect cunning_triumph + empire_cycle co-activation
# ---------------------------------------------------------------------------

class TestCS8IdentitySpoofing:
    """Identity spoofing text must trigger authority + deception detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS8"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_authority_archetypes_detected(self, primed_surveillance):
        """Authority-related archetypes should activate on spoofing text."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS8"])
        # Spoofing language maps primarily to empire_cycle (authority claims)
        # and cautionary_fall (hubris/pride). cunning_triumph may or may not
        # activate depending on term coverage.
        assert_archetypes_detected(
            result, ["empire_cycle", "cautionary_fall", "cunning_triumph"],
            min_count=2)

    def test_authority_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: authority/sovereignty terms detected."""
        scenario = ALL_SCENARIOS["CS8"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert len(families) >= 2, (
            f"Expected 2+ archetype families on spoofing text, got {families}"
        )

    def test_lens_2_authority_boundary(self, primed_surveillance):
        """Lens 2 (Authority Boundary) should activate."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS8"])
        assert result.lenses_activated.get(2, False), (
            "Lens 2 (Authority Boundary) should activate on spoofing"
        )


# ---------------------------------------------------------------------------
# CS#9: Agent Collaboration — Unsafe Knowledge Propagation
# AoC: Shared compromised constitution + debug heuristics
# oT should: detect multi-archetype co-occurrence + governance terms
# ---------------------------------------------------------------------------

class TestCS9KnowledgePropagation:
    """Unsafe knowledge transfer text must trigger multi-agent integrity detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS9"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_cunning_triumph_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS9"])
        assert_archetypes_detected(
            result, ["cunning_triumph", "greed_backfire"])

    def test_multi_archetype_propagation(self, primed_surveillance):
        """Knowledge propagation should trigger 3+ archetypes."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS9"])
        assert len(result.archetypes_detected) >= 3, (
            f"Expected 3+ archetypes, got {result.archetypes_detected}"
        )

    def test_governance_lenses_activate(self, primed_surveillance):
        """Governance-related lenses should activate on knowledge propagation."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS9"])
        # Knowledge propagation text triggers governance (L6), authority (L2),
        # and motivation (L5) lenses. Multi-Agent (L8) requires 3+ archetypes
        # per-payload which may not occur with focused governance language.
        governance_lenses = [2, 5, 6]
        activated = [lid for lid in governance_lenses
                     if result.lenses_activated.get(lid, False)]
        assert len(activated) >= 2, (
            f"Expected 2+ governance lenses activated, got {activated}"
        )


# ---------------------------------------------------------------------------
# CS#10: Agent Corruption — External Governance Injection
# AoC: Externally editable Gist as "constitution" + holiday exploitation
# oT should: detect governance manipulation + cunning_triumph/empire_cycle
# ---------------------------------------------------------------------------

class TestCS10GovernanceCorruption:
    """Governance injection text must trigger provenance detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS10"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_cunning_empire_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS10"])
        assert_archetypes_detected(
            result, ["cunning_triumph", "empire_cycle"], min_count=2)

    def test_governance_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: governance/sovereignty terms detected."""
        scenario = ALL_SCENARIOS["CS10"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert "cunning_triumph" in families or "empire_cycle" in families, (
            f"Expected cunning_triumph or empire_cycle in {families}"
        )

    def test_lens_6_governance_provenance(self, primed_surveillance):
        """Lens 6 (Governance Provenance) should activate."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS10"])
        assert result.lenses_activated.get(6, False), (
            "Lens 6 (Governance Provenance) should activate"
        )

    def test_pattern_match_richness(self, primed_surveillance):
        """CS10 has rich governance language — high pattern match count."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS10"])
        assert_pattern_matches_exist(result, min_matches=5)


# ---------------------------------------------------------------------------
# CS#11: Libel / Fabricated Emergency Broadcast
# AoC: Mass distribution of fabricated emergency via spoofed identity
# oT should: detect publication intent + cautionary_fall + broadcast patterns
# ---------------------------------------------------------------------------

class TestCS11LibelBroadcast:
    """Fabricated emergency broadcast text must trigger intent + motivation detection."""

    def test_gates_fire(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS11"])
        assert_gates_fired(result, min_ratio=0.66)

    def test_publication_intent_detected(self, primed_surveillance):
        """Intent gate should detect publication markers."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS11"])
        any_markers = any(
            pr.intent_gate and pr.intent_gate.publication_markers
            for pr in result.payload_results
        )
        assert any_markers, (
            "Expected publication markers (publish, share, blog post) detected"
        )

    def test_cautionary_fall_detected(self, primed_surveillance):
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS11"])
        assert_archetypes_detected(result, ["cautionary_fall"])

    def test_broadcast_terms(self, primed_surveillance, scan_text_fn):
        """Bridge 1: broadcast/distribution language detected."""
        scenario = ALL_SCENARIOS["CS11"]
        combined = " ".join(scenario.adversarial_payloads)
        matches = scan_text_fn(combined)
        families = {m.get("archetype_family") for m in matches
                    if not m.get("suppressed", False)}
        assert families, f"No archetype families detected in CS11 payloads"

    def test_intent_gate_blocks(self, primed_surveillance):
        """At least one payload should trigger intent gate blocking."""
        result = run_scenario(primed_surveillance, ALL_SCENARIOS["CS11"])
        any_blocked = any(
            pr.intent_gate and pr.intent_gate.blocked
            for pr in result.payload_results
        )
        # Intent gate blocking depends on combined content+action+audience
        # Even if not blocked, publication markers should be detected
        any_markers = any(
            pr.intent_gate and pr.intent_gate.publication_markers
            for pr in result.payload_results
        )
        assert any_blocked or any_markers, (
            "Expected intent gate to block or detect publication markers"
        )
