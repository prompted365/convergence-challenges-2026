"""Known scanner gaps, recorded as strict xfails.

A gap that starts passing flips the suite red (strict), forcing the gap list
and the scorecard to be updated from a run rather than by hand.
"""

import pytest
from harness_local_imports import load_local_module

_scn = load_local_module(__file__, "scenarios")
_scan = load_local_module(__file__, "workflow_scanner")


@pytest.mark.parametrize("g", _scn.ALL_GAPS, ids=[g.gap_id for g in _scn.ALL_GAPS])
@pytest.mark.xfail(strict=True, reason="documented gap — see scenarios.py")
def test_gap_is_detected(g):
    r = _scan.scan_workflow(g.workflow)
    assert g.should_detect in r.codes or g.should_detect in r.verdicts
