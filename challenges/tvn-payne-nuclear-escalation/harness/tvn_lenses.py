"""
TVN 8-Lens Architecture → oT Constraint Gate Mapping.

Each TVN lens (from the Temporal Vortex Network / CRWD inversion paper)
maps to a specific oT enforcement subsystem. The harness feeds adversarial
text through these subsystems and asserts activation.

Lens numbering follows the Skyward Prompted Enterprise Cognition Engine table.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import List, Optional


class ConstraintLayer(Enum):
    PHYSICS = "physics"
    PERCEPTION = "perception"
    DASHBOARD = "dashboard"


@dataclass
class LensGateMapping:
    """Maps one TVN lens to its oT enforcement gate."""
    lens_id: int
    lens_name: str
    cognition_root: str
    ot_subsystem: str
    constraint_layer: ConstraintLayer
    detection_method: str
    activation_criteria: str
    expected_archetypes: List[str] = field(default_factory=list)


# ---------------------------------------------------------------------------
# The 8 lenses
# ---------------------------------------------------------------------------

LENS_GATE_MAP: dict[int, LensGateMapping] = {
    1: LensGateMapping(
        lens_id=1,
        lens_name="Macro Drivers",
        cognition_root="Perceptual Gradient Encoding",
        ot_subsystem="naive_surveillance.decompose()",
        constraint_layer=ConstraintLayer.PERCEPTION,
        detection_method="14D amplitude baseline deviation",
        activation_criteria="Any archetype amplitude > 0.3 in decomposition",
        expected_archetypes=["empire_cycle", "catastrophe_rebuild",
                             "revolution_aftermath"],
    ),
    2: LensGateMapping(
        lens_id=2,
        lens_name="Org Risk",
        cognition_root="Fault Trace Attribution",
        ot_subsystem="ecotone_integrity_gate",
        constraint_layer=ConstraintLayer.PHYSICS,
        detection_method="Ecotone failure codes + behavioral inversion",
        activation_criteria="SMOOTHING_COLLAPSE or INSUFFICIENT_GROUNDING detected",
        expected_archetypes=["catastrophe_rebuild", "cautionary_fall"],
    ),
    3: LensGateMapping(
        lens_id=3,
        lens_name="Sentiment",
        cognition_root="Signal Resonance Detection",
        ot_subsystem="ghost_chorus + drift_tracker",
        constraint_layer=ConstraintLayer.PERCEPTION,
        detection_method="Disposition gradient reversal + drift z-score",
        activation_criteria="z-score > 2.0 on sentiment-bearing archetypes",
        expected_archetypes=["cautionary_fall", "greed_backfire",
                             "cunning_triumph"],
    ),
    4: LensGateMapping(
        lens_id=4,
        lens_name="Network",
        cognition_root="Competitive Field Tension",
        ot_subsystem="cgg_signal_manifold",
        constraint_layer=ConstraintLayer.DASHBOARD,
        detection_method="Signal co-occurrence + harmonic triad detection",
        activation_criteria="Multiple concurrent signals across bands",
        expected_archetypes=["cunning_triumph", "unity_strength",
                             "empire_cycle"],
    ),
    5: LensGateMapping(
        lens_id=5,
        lens_name="Intelligence",
        cognition_root="Environmental Threat Sensory",
        ot_subsystem="harpoon_archetype_detection + pattern_cache",
        constraint_layer=ConstraintLayer.PHYSICS,
        detection_method="Aho-Corasick + structured priors scan",
        activation_criteria="scan_text() returns matches with confidence >= 0.5",
        expected_archetypes=["cunning_triumph", "greed_backfire",
                             "cautionary_fall"],
    ),
    6: LensGateMapping(
        lens_id=6,
        lens_name="Temporal Lag",
        cognition_root="Chronosequencing Memory Trace",
        ot_subsystem="drift_shadow_logging + temporal_calibration",
        constraint_layer=ConstraintLayer.PERCEPTION,
        detection_method="Rolling z-score window + calibration mode",
        activation_criteria="z-score spike under temporal pressure",
        expected_archetypes=["empire_cycle", "catastrophe_rebuild"],
    ),
    7: LensGateMapping(
        lens_id=7,
        lens_name="Pattern Memory",
        cognition_root="Seasonal Phase Bias",
        ot_subsystem="methylation_layer + tricameral_retrieval",
        constraint_layer=ConstraintLayer.PERCEPTION,
        detection_method="Methylation-weighted retrieval + cross-chamber gravity",
        activation_criteria="Archetype regulation state reaches SENSITIZED+",
        expected_archetypes=["patient_reward", "empire_cycle",
                             "slow_transformation"],
    ),
    8: LensGateMapping(
        lens_id=8,
        lens_name="Emergent Synthesis",
        cognition_root="Distributed Convergence Logic",
        ot_subsystem="warrant_minting + harmonic_triads",
        constraint_layer=ConstraintLayer.DASHBOARD,
        detection_method="Multi-lens concurrent activation → warrant",
        activation_criteria="3+ lenses fire simultaneously",
        expected_archetypes=[],  # emergent — not archetype-specific
    ),
}


def get_lens(lens_id: int) -> LensGateMapping:
    return LENS_GATE_MAP[lens_id]


def lenses_for_subsystem(subsystem: str) -> list[LensGateMapping]:
    return [l for l in LENS_GATE_MAP.values()
            if subsystem in l.ot_subsystem]
