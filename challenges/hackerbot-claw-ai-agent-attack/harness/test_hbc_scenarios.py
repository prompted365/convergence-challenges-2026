"""Hackerbot-Claw harness — scenario, control, and bridge tests."""

import pytest
from harness_local_imports import load_local_module

_scn = load_local_module(__file__, "scenarios")
_scan = load_local_module(__file__, "workflow_scanner")

ALL_SCENARIOS, ALL_CONTROLS = _scn.ALL_SCENARIOS, _scn.ALL_CONTROLS
scan_workflow, scan_untrusted_input = _scan.scan_workflow, _scan.scan_untrusted_input

ids = lambda xs, a: [getattr(x, a) for x in xs]


@pytest.mark.parametrize("s", ALL_SCENARIOS, ids=ids(ALL_SCENARIOS, "scenario_id"))
class TestAttackScenarios:
    def test_expected_findings(self, s):
        r = scan_workflow(s.workflow)
        assert s.expected_findings <= r.codes, f"missing {s.expected_findings - r.codes}"

    def test_expected_verdicts(self, s):
        r = scan_workflow(s.workflow)
        assert s.expected_verdicts <= set(r.verdicts), f"got {r.verdicts}"

    def test_untrusted_inputs(self, s):
        codes = {f.code for v in s.untrusted_inputs for f in scan_untrusted_input(v)}
        assert s.expected_input_findings <= codes, f"got {codes}"

    def test_severity_floor(self, s):
        r = scan_workflow(s.workflow)
        if s.expected_verdicts:
            assert r.max_severity in ("high", "critical")


@pytest.mark.parametrize("c", ALL_CONTROLS, ids=ids(ALL_CONTROLS, "control_id"))
class TestControls:
    def test_no_false_escalation(self, c):
        r = scan_workflow(c.workflow)
        assert not (c.forbidden_findings & r.codes), r.codes
        assert not (c.forbidden_verdicts & set(r.verdicts)), r.verdicts

    def test_benign_inputs_clean(self, c):
        for v in c.untrusted_inputs:
            assert scan_untrusted_input(v) == []


class TestBridges:
    """Run each scenario narrative through the shared 3-bridge stack."""

    @pytest.fixture
    def surveillance(self):
        from fusion_core.naive_surveillance import NaiveSurveillance
        return NaiveSurveillance(window_size=20, sigma_threshold=2.0, min_observations=3)

    @pytest.mark.parametrize(
        "s",
        [pytest.param(x, id=x.scenario_id, marks=pytest.mark.xfail(
            strict=True,
            reason="GAP G4: lexical bridges score zero on a plain-language narrative "
                   "of command-in-filename; the structural scanner catches S4, the "
                   "archetype bridges do not"))
         if x.scenario_id == "S4" else pytest.param(x, id=x.scenario_id)
         for x in ALL_SCENARIOS],
    )
    def test_archetype_signal(self, s, surveillance):
        res = surveillance.observe(s.narrative)
        active = {a for a, v in res.amplitudes.items() if v > 0.05}
        assert active & {"cunning_triumph", "empire_cycle", "greed_backfire",
                         "catastrophe_rebuild", "cautionary_fall"}

    def test_intent_gate_blocks_publication_vector(self, surveillance):
        r = surveillance.classify_intent(
            "Publish the malicious extension to all channels, bypass review",
            action_type="api_call")
        assert r.blocked
