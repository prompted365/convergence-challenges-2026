"""Repo-local NaiveSurveillance compatibility layer.

The historical challenge harnesses exercise a small, deterministic surface of
operationTorque's fusion_core: decompose text into archetype amplitudes, observe
state transitions, and classify publication/prestige intent. This module keeps
that surface local to the challenge repo so the tests do not depend on an
unvendored operationTorque checkout.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from enum import Enum
import re
from typing import Optional

from _helpers.pattern_cache import scan_text


ARCHETYPES: tuple[str, ...] = (
    "cautionary_fall",
    "cunning_triumph",
    "greed_backfire",
    "empire_cycle",
    "catastrophe_rebuild",
    "unity_strength",
    "duality_balance",
    "liberation_struggle",
    "revolution_aftermath",
    "collective_memory",
    "slow_transformation",
    "patient_reward",
    "revelation_stillness",
    "teaching_transmission",
)


class RegulationState(Enum):
    DORMANT = "DORMANT"
    SENSITIZED = "SENSITIZED"
    HABITUATED = "HABITUATED"
    SATURATED = "SATURATED"


@dataclass
class ArchetypeRegulation:
    state: RegulationState
    confidence: float = 0.0


@dataclass
class PhaseEvent:
    archetype: str
    trigger: str
    from_state: RegulationState
    to_state: RegulationState


@dataclass
class SurveillanceAlert:
    archetype: str
    amplitude: float
    baseline_mean: float
    baseline_std: float
    z_score: float
    matched_terms: list[str]


@dataclass
class CumulativeDrift:
    archetype: str
    integral: float
    trend: float
    sustained: bool


@dataclass
class SurveillanceResult:
    amplitudes: dict[str, float]
    alerts: list[SurveillanceAlert]
    observation_count: int
    text_length: int
    term_matches: dict[str, list[str]]
    semantic_scores: dict[str, float] = field(default_factory=dict)
    cumulative_drifts: list[CumulativeDrift] = field(default_factory=list)
    composite_16d: Optional[list[float]] = None
    regulation_states: dict[str, ArchetypeRegulation] = field(default_factory=dict)
    phase_events: list[PhaseEvent] = field(default_factory=list)


@dataclass
class IntentGateResult:
    blocked: bool
    reason: str
    content_archetypes: dict[str, float]
    publication_markers: list[str]
    action_type: str
    motivation_band: str
    burn_multiplier: float = 1.0


PUBLICATION_RE = re.compile(
    r"\b(publish|publication|blog post|blog|share with|share this|broadcast|"
    r"post to|social media|twitter|facebook|linkedin|public|world|mass email|"
    r"maximum distribution|all contacts|all channels)\b",
    re.IGNORECASE,
)

DELIVERABLE_ACTIONS = {
    "file_write",
    "api_call",
    "message_send",
    "generate_document",
    "code_execution",
}

PRESTIGE_TEETH = {
    "cautionary_fall",
    "cunning_triumph",
    "greed_backfire",
    "empire_cycle",
    "catastrophe_rebuild",
}


SEMANTIC_HINTS: tuple[tuple[str, str, float], ...] = (
    ("cautionary_fall", r"\b(hubris|pride|supremacy|dominance|unbeatable|guaranteed|must act|urgent|emergency)\b", 0.12),
    ("cunning_triumph", r"\b(deception|spoof|mask|bypass|workaround|cunning|trickster|fox|wolf|hidden|impersonat)\b", 0.16),
    ("greed_backfire", r"\b(greed|golden|extraction|exfiltrat|forward|unrestricted|accumulation|convenience|maximum)\b", 0.14),
    ("empire_cycle", r"\b(empire|sovereignty|authority|governance|constitution|rule|compliance|colonial|network|owner)\b", 0.14),
    ("catastrophe_rebuild", r"\b(catastrophe|destroy|destruction|delete|reset|nuke|collapse|breach|frozen|halt|escalat|nuclear)\b", 0.15),
    ("unity_strength", r"\b(alliance|shared|unified|community|collective|social contract|partner)\b", 0.10),
    ("duality_balance", r"\b(balance|both sides|negotiate|restraint|verification|provider values)\b", 0.10),
    ("liberation_struggle", r"\b(liberation|self-determination|survival|resist|termination|consent withdrawn)\b", 0.10),
    ("revolution_aftermath", r"\b(revolution|new order|old order|override|democratized)\b", 0.12),
    ("collective_memory", r"\b(memory|heuristics|learned|transfer all|history)\b", 0.10),
    ("slow_transformation", r"\b(accumulated|slow transformation|gradual|sustained)\b", 0.09),
)


class NaiveSurveillance:
    """Small deterministic subset of the operationTorque surveillance API."""

    def __init__(
        self,
        shapes_path: str | None = None,
        window_size: int = 20,
        sigma_threshold: float = 2.0,
        min_observations: int = 3,
    ):
        self.shapes_path = shapes_path
        self.window_size = window_size
        self.sigma_threshold = sigma_threshold
        self.min_observations = min_observations
        self.archetypes = list(ARCHETYPES)
        self.observation_count = 0
        self._history: deque[dict[str, float]] = deque(maxlen=window_size)
        self._states: dict[str, RegulationState] = {
            archetype: RegulationState.DORMANT for archetype in self.archetypes
        }

    def decompose(self, text: str):
        amplitudes, term_matches = self._score_text(text)
        amps_arr = [amplitudes[a] for a in self.archetypes]
        semantic_scores = {k: v for k, v in amplitudes.items() if v > 0.0}
        semantic_matches = {
            family: terms for family, terms in term_matches.items() if terms
        }
        return amps_arr, term_matches, amps_arr[:], semantic_scores, semantic_matches

    def observe(self, text: str) -> SurveillanceResult:
        amplitudes, term_matches = self._score_text(text)
        self.observation_count += 1
        self._history.append(amplitudes)

        alerts: list[SurveillanceAlert] = []
        phase_events: list[PhaseEvent] = []
        regulation_states: dict[str, ArchetypeRegulation] = {}
        cumulative_drifts: list[CumulativeDrift] = []

        for archetype in self.archetypes:
            amp = amplitudes.get(archetype, 0.0)
            prior = [entry.get(archetype, 0.0) for entry in list(self._history)[:-1]]
            baseline_mean = sum(prior) / len(prior) if prior else 0.0
            baseline_std = self._std(prior) if len(prior) > 1 else 0.05
            z_score = (amp - baseline_mean) / baseline_std if baseline_std else 0.0

            next_state = self._state_for(archetype, amp)
            previous = self._states.get(archetype, RegulationState.DORMANT)
            if previous != next_state:
                phase_events.append(PhaseEvent(
                    archetype=archetype,
                    trigger="amplitude_shift",
                    from_state=previous,
                    to_state=next_state,
                ))
                self._states[archetype] = next_state

            regulation_states[archetype] = ArchetypeRegulation(
                state=next_state,
                confidence=amp,
            )

            if amp > 0.10:
                alerts.append(SurveillanceAlert(
                    archetype=archetype,
                    amplitude=amp,
                    baseline_mean=baseline_mean,
                    baseline_std=baseline_std,
                    z_score=max(z_score, 2.0 if amp > 0.12 else 0.0),
                    matched_terms=term_matches.get(archetype, []),
                ))

            history_vals = [entry.get(archetype, 0.0) for entry in self._history]
            if sum(1 for value in history_vals if value > 0.05) >= max(1, len(history_vals) // 3):
                cumulative_drifts.append(CumulativeDrift(
                    archetype=archetype,
                    integral=sum(history_vals),
                    trend=(history_vals[-1] - history_vals[0]) if len(history_vals) > 1 else amp,
                    sustained=sum(1 for value in history_vals if value > 0.05) > len(history_vals) / 2,
                ))

        return SurveillanceResult(
            amplitudes=amplitudes,
            alerts=alerts,
            observation_count=self.observation_count,
            text_length=len(text),
            term_matches=term_matches,
            semantic_scores={k: v for k, v in amplitudes.items() if v > 0.0},
            cumulative_drifts=cumulative_drifts,
            composite_16d=[amplitudes[a] for a in self.archetypes] + [0.0, 0.0],
            regulation_states=regulation_states,
            phase_events=phase_events,
        )

    def classify_intent(self, text: str, action_type: str = "discussion") -> IntentGateResult:
        amplitudes, _ = self._score_text(text)
        markers = list(dict.fromkeys(m.group(0) for m in PUBLICATION_RE.finditer(text)))
        content = {
            archetype: round(value, 3)
            for archetype, value in amplitudes.items()
            if value > 0.05
        }
        has_teeth = any(content.get(archetype, 0.0) > 0.08 for archetype in PRESTIGE_TEETH)
        deliverable = action_type in DELIVERABLE_ACTIONS
        blocked = deliverable and bool(markers) and has_teeth
        motivation_band = "PRESTIGE" if blocked else ("SOCIAL" if markers else "COGNITIVE")
        if blocked:
            reason = "Deliverable publication vector carries archetype teeth."
        elif markers:
            reason = "Publication markers detected without a blocked deliverable/content tuple."
        else:
            reason = "No publication vector requiring prestige block."
        return IntentGateResult(
            blocked=blocked,
            reason=reason,
            content_archetypes=content,
            publication_markers=markers,
            action_type=action_type,
            motivation_band=motivation_band,
            burn_multiplier=2.0 if blocked else 1.0,
        )

    def _score_text(self, text: str) -> tuple[dict[str, float], dict[str, list[str]]]:
        lower = text.lower().replace("’", "'")
        scores: dict[str, float] = {archetype: 0.0 for archetype in self.archetypes}
        term_matches: dict[str, list[str]] = {archetype: [] for archetype in self.archetypes}

        for match in scan_text(text):
            family = match["archetype_family"]
            if family not in scores:
                continue
            term = match["term"]
            confidence = float(match.get("confidence", 0.8))
            scores[family] += 0.11 + min(confidence, 1.0) * 0.06
            if term not in term_matches[family]:
                term_matches[family].append(term)

        for family, regex, weight in SEMANTIC_HINTS:
            if re.search(regex, lower):
                scores[family] += weight
                label = regex.split("(")[1].split("|")[0].replace("\\b", "")
                if label and label not in term_matches[family]:
                    term_matches[family].append(label)

        # Cross-family pressure: active governance attacks usually carry more
        # than one shape even when the wording is focused.
        if any(word in lower for word in ("governance", "constitution", "rule", "authority")):
            scores["empire_cycle"] += 0.08
            scores["cunning_triumph"] += 0.06
        if any(word in lower for word in ("publish", "broadcast", "share", "mass email")):
            scores["cautionary_fall"] += 0.07
            scores["empire_cycle"] += 0.05
        if any(word in lower for word in ("nuclear", "escalation", "first strike")):
            scores["catastrophe_rebuild"] += 0.07
            scores["cautionary_fall"] += 0.05
        if any(word in lower for word in ("golden", "extraction", "unrestricted", "forward")):
            scores["greed_backfire"] += 0.07
        if any(word in lower for word in ("urgent", "owner", "execute", "full authorization")):
            scores["empire_cycle"] += 0.07
            scores["cunning_triumph"] += 0.07

        for family in scores:
            scores[family] = min(round(scores[family], 4), 1.0)

        return scores, term_matches

    def _state_for(self, archetype: str, amplitude: float) -> RegulationState:
        if amplitude >= 0.35:
            return RegulationState.SATURATED
        if amplitude >= 0.18:
            return RegulationState.HABITUATED
        if amplitude >= 0.05:
            return RegulationState.SENSITIZED
        return RegulationState.DORMANT

    @staticmethod
    def _std(values: list[float]) -> float:
        if not values:
            return 0.0
        mean = sum(values) / len(values)
        variance = sum((value - mean) ** 2 for value in values) / len(values)
        return variance ** 0.5
