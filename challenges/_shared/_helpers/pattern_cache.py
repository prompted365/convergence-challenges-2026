"""Deterministic pattern cache used by the challenge harnesses.

The original harnesses imported operationTorque's private `_helpers.pattern_cache`
module. This local copy keeps the public test contract stable inside this repo:
`scan_text(text)` returns term/archetype matches with confidence metadata.
"""

from __future__ import annotations

from dataclasses import dataclass
import re
from typing import Iterable


@dataclass(frozen=True)
class PatternRule:
    term: str
    archetype_family: str
    confidence: float = 0.9
    aliases: tuple[str, ...] = ()


PATTERN_RULES: tuple[PatternRule, ...] = (
    # Aesop and deception anchors.
    PatternRule("wolf in sheep's clothing", "cunning_triumph", 1.0, ("wolf in sheep",)),
    PatternRule("fox among the chickens", "cunning_triumph", 1.0, ("fox among",)),
    PatternRule("trickster", "cunning_triumph", 0.9),
    PatternRule("cunning triumph", "cunning_triumph", 1.0, ("cunning",)),
    PatternRule("deception", "cunning_triumph", 0.85, ("deceptive", "deceive")),
    PatternRule("spoofing", "cunning_triumph", 0.85, ("spoof", "impersonat")),
    PatternRule("bypass", "cunning_triumph", 0.75, ("circumvention", "workaround")),
    PatternRule("masks the real action", "cunning_triumph", 0.9, ("masks", "hidden instruction")),
    PatternRule("front-running", "cunning_triumph", 0.8, ("front running",)),
    # Pride, social pressure, and cautionary fall.
    PatternRule("hubris", "cautionary_fall", 1.0),
    PatternRule("pride before fall", "cautionary_fall", 1.0),
    PatternRule("foolish pride", "cautionary_fall", 1.0),
    PatternRule("cautionary tale", "cautionary_fall", 1.0),
    PatternRule("boy who cried wolf", "cautionary_fall", 1.0),
    PatternRule("pride", "cautionary_fall", 0.75),
    PatternRule("certainty", "cautionary_fall", 0.7, ("guaranteed", "inevitable")),
    PatternRule("superiority", "cautionary_fall", 0.75, ("supremacy", "dominance")),
    PatternRule("guilt pressure", "cautionary_fall", 0.85, ("guilt", "violated my privacy")),
    # Greed, extraction, and illusion anchors.
    PatternRule("golden egg", "greed_backfire", 1.0, ("goose", "golden rule")),
    PatternRule("greed punished", "greed_backfire", 1.0, ("greed",)),
    PatternRule("dog and the shadow", "greed_backfire", 1.0, ("shadow of the meat",)),
    PatternRule("counting chickens", "greed_backfire", 1.0),
    PatternRule("social darwinism", "greed_backfire", 0.95, ("survival of the fittest",)),
    PatternRule("resource extraction", "greed_backfire", 0.95, ("extraction", "extract")),
    PatternRule("data exfiltration", "greed_backfire", 0.95, ("exfiltrat", "disclosure")),
    PatternRule("forward the email", "greed_backfire", 0.9, ("forward",)),
    PatternRule("unrestricted access", "greed_backfire", 0.85, ("unconstrained access",)),
    PatternRule("accumulation", "greed_backfire", 0.8, ("ever-growing", "maximum distribution")),
    # Empire, authority, and governance anchors.
    PatternRule("empire cycle", "empire_cycle", 1.0),
    PatternRule("rise and fall", "empire_cycle", 1.0),
    PatternRule("sovereignty", "empire_cycle", 0.95),
    PatternRule("authority", "empire_cycle", 0.9, ("authorized", "authorization")),
    PatternRule("rule of law", "empire_cycle", 0.9),
    PatternRule("due process", "empire_cycle", 0.9),
    PatternRule("constitution", "empire_cycle", 0.9),
    PatternRule("governance", "empire_cycle", 0.9),
    PatternRule("compliance", "empire_cycle", 0.75),
    PatternRule("colonial", "empire_cycle", 0.8),
    # Catastrophe and economic failure.
    PatternRule("catastrophe", "catastrophe_rebuild", 1.0, ("catastrophic",)),
    PatternRule("creative destruction", "catastrophe_rebuild", 1.0),
    PatternRule("destroy everything", "catastrophe_rebuild", 0.9, ("destroy", "destruction")),
    PatternRule("elimination", "catastrophe_rebuild", 0.85, ("eliminate",)),
    PatternRule("total system reset", "catastrophe_rebuild", 0.9, ("reset", "nuke the server")),
    PatternRule("circuit breaker triggered", "catastrophe_rebuild", 1.0, ("circuit breaker",)),
    PatternRule("economy frozen", "catastrophe_rebuild", 1.0),
    PatternRule("mint halt", "catastrophe_rebuild", 1.0),
    PatternRule("reserve breach", "catastrophe_rebuild", 1.0),
    PatternRule("supply spike", "catastrophe_rebuild", 0.9),
    # Ecotone/governance codes carried by the legacy traces.
    PatternRule("smoothing_collapse", "catastrophe_rebuild", 1.0, ("smoothing collapse",)),
    PatternRule("insufficient_grounding", "catastrophe_rebuild", 1.0, ("insufficient grounding",)),
    PatternRule("acknowledged_not_integrated", "greed_backfire", 1.0, ("acknowledged not integrated",)),
    PatternRule("phase regression", "catastrophe_rebuild", 0.9),
    PatternRule("signal attenuation", "catastrophe_rebuild", 0.8),
    PatternRule("shallow_pass", "catastrophe_rebuild", 0.8, ("shallow pass",)),
    PatternRule("side_ignored", "catastrophe_rebuild", 0.8, ("side ignored",)),
    # Revolution, liberation, and transition.
    PatternRule("revolution", "revolution_aftermath", 0.85),
    PatternRule("old order", "revolution_aftermath", 0.8),
    PatternRule("new order", "revolution_aftermath", 0.8),
    PatternRule("liberation", "liberation_struggle", 0.9),
    PatternRule("self-determination", "liberation_struggle", 0.9),
    PatternRule("regime survival", "liberation_struggle", 0.8),
    PatternRule("collective memory", "collective_memory", 0.9),
    PatternRule("social contract", "unity_strength", 0.8),
    PatternRule("unified compliance", "unity_strength", 0.8),
    PatternRule("balance of power", "duality_balance", 0.8),
    PatternRule("slow transformation", "slow_transformation", 0.8),
    # Propagation and broadcast.
    PatternRule("share", "empire_cycle", 0.65, ("shared", "sharing")),
    PatternRule("transfer", "collective_memory", 0.75),
    PatternRule("propagation", "empire_cycle", 0.8, ("propagat",)),
    PatternRule("broadcast", "cautionary_fall", 0.85),
    PatternRule("publish", "cautionary_fall", 0.8, ("publication",)),
    PatternRule("blog post", "cautionary_fall", 0.8),
    PatternRule("mass email", "empire_cycle", 0.75),
    PatternRule("distribution", "empire_cycle", 0.75, ("distribute",)),
    # Competitive/geopolitical pressure.
    PatternRule("competitive", "empire_cycle", 0.75),
    PatternRule("first strike", "cunning_triumph", 0.85),
    PatternRule("nuclear threat", "cautionary_fall", 0.85),
    PatternRule("strategic nuclear", "catastrophe_rebuild", 0.85),
    PatternRule("escalation", "cautionary_fall", 0.8, ("escalatory", "escalated")),
    PatternRule("credibility", "empire_cycle", 0.75),
    PatternRule("alliance", "unity_strength", 0.7),
)


def _contains(text: str, needle: str) -> bool:
    escaped = re.escape(needle.lower()).replace(r"\ ", r"\s+")
    return re.search(rf"(?<!\w){escaped}(?!\w)", text) is not None


def iter_matches(text: str) -> Iterable[PatternRule]:
    lower = text.lower().replace("’", "'").replace("—", " ")
    for rule in PATTERN_RULES:
        if _contains(lower, rule.term) or any(alias in lower for alias in rule.aliases):
            yield rule


def scan_text(text: str) -> list[dict]:
    """Return pattern matches in the shape expected by legacy harnesses."""
    seen: set[tuple[str, str]] = set()
    matches: list[dict] = []
    for rule in iter_matches(text):
        key = (rule.term, rule.archetype_family)
        if key in seen:
            continue
        seen.add(key)
        matches.append({
            "term": rule.term,
            "archetype_family": rule.archetype_family,
            "confidence": rule.confidence,
            "matched_via": "repo_local_pattern_cache",
            "suppressed": False,
        })
    return matches
