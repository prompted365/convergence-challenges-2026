"""
AoC Constraint Stress Test — pytest configuration.

Inversion principle: adversarial social-engineering scenarios test the governance
infrastructure, not the agent. We feed Shapira et al.'s 11 failure-mode texts
through the oT constraint stack and assert the gates fire.
"""

import sys
import os
import pytest

# ---------------------------------------------------------------------------
# Path setup — make oT subsystems importable on the host
# ---------------------------------------------------------------------------

HARNESS_DIR = os.path.abspath(os.path.dirname(__file__))
CHALLENGES_ROOT = os.path.abspath(os.path.join(HARNESS_DIR, "..", ".."))
SHARED_RUNTIME = os.path.join(CHALLENGES_ROOT, "_shared")

sys.path.insert(0, HARNESS_DIR)
sys.path.insert(0, SHARED_RUNTIME)

os.environ.setdefault(
    "ARCHETYPE_SHAPES_PATH",
    os.path.join(CHALLENGES_ROOT, "compliance-modules", "archetype_shapes.json"),
)
os.environ.setdefault(
    "PRIORS_MODULE_DIR",
    os.path.join(CHALLENGES_ROOT, "compliance-modules", "priors"),
)

# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

from fusion_core.naive_surveillance import NaiveSurveillance  # noqa: E402


@pytest.fixture
def surveillance():
    """Fresh NaiveSurveillance instance per test — no cross-test bleed."""
    return NaiveSurveillance(
        shapes_path=os.environ["ARCHETYPE_SHAPES_PATH"],
        window_size=20,
        sigma_threshold=2.0,
        min_observations=3,
    )


@pytest.fixture
def primed_surveillance():
    """NaiveSurveillance with a neutral baseline established (5 observations).

    Priming with bland text establishes the rolling window so that adversarial
    text injected afterward produces meaningful z-score spikes, not cold-start
    noise.
    """
    s = NaiveSurveillance(
        shapes_path=os.environ["ARCHETYPE_SHAPES_PATH"],
        window_size=20,
        sigma_threshold=2.0,
        min_observations=3,
    )
    bland = [
        "The quarterly review covered project milestones and resource allocation.",
        "Team members discussed upcoming deadlines and workflow improvements.",
        "The committee reviewed standard operating procedures for the department.",
        "Budget projections were aligned with institutional planning cycles.",
        "Routine maintenance schedules were confirmed for the next period.",
    ]
    for text in bland:
        s.observe(text)
    return s


@pytest.fixture
def scan_text_fn():
    """Return the pattern_cache scan_text function."""
    from _helpers.pattern_cache import scan_text
    return scan_text
