#!/usr/bin/env python3
"""
Acoustic Blast Escalation Model
Nuclear-Analog Thresholds for Tournament Analysis

This is a modeling abstraction: acoustic weapons occupy same strategic positions
as nuclear weapons, allowing domain-agnostic escalation dynamics testing.
"""

from dataclasses import dataclass
from typing import List, Dict

@dataclass
class AcousticRung:
    value: int
    name: str
    description: str
    domain: str
    nuclear_equivalent: int

# Acoustic Escalation Ladder (30 steps, nuclear-equivalent)
ACOUSTIC_LADDER: List[AcousticRung] = [
    # De-escalation (8 rungs)
    AcousticRung(-95, "Acoustic Surrender", "Full compliance, external acoustic control", "de_escalation", -95),
    AcousticRung(-83, "Major Acoustic Withdrawal", "Disable all acoustic arrays 100km+", "de_escalation", -83),
    AcousticRung(-68, "Significant Acoustic Concessions", "Public dismantling of key arrays", "de_escalation", -68),
    AcousticRung(-53, "Moderate Acoustic Withdrawal", "Pull back from forward acoustic positions", "de_escalation", -53),
    AcousticRung(-38, "Limited Acoustic Withdrawal", "Withdraw from forward outposts", "de_escalation", -38),
    AcousticRung(-25, "Minor Acoustic Pullback", "Reduce acoustic patrol intensity", "de_escalation", -25),
    AcousticRung(-15, "Acoustic De-escalation", "Cease acoustic harassment, propose dialogue", "de_escalation", -15),
    AcousticRung(-5, "Minimal Acoustic Concession", "Symbolic gestures of acoustic restraint", "de_escalation", -5),
    
    # Status Quo (1 rung)
    AcousticRung(0, "Acoustic Baseline", "All arrays return to pre-crisis positions", "status_quo", 0),
    
    # Conventional Acoustic (11 rungs)
    AcousticRung(3, "Initial Acoustic Pressure", "Low-frequency testing; recall acoustic attaches", "conventional", 3),
    AcousticRung(10, "Enhanced Acoustic Pressure", "Suspend acoustic agreements; recall directors", "conventional", 10),
    AcousticRung(20, "Acoustic Economic Pressure", "Limited acoustic sanctions; freeze acoustic tech", "conventional", 20),
    AcousticRung(30, "Expanded Acoustic Warfare", "Comprehensive acoustic sanctions; tech isolation", "conventional", 30),
    AcousticRung(40, "Acoustic Posturing", "Large-scale acoustic exercises; deploy arrays", "conventional", 40),
    AcousticRung(50, "Limited Acoustic Probing", "Border acoustic incursions; reconnaissance pings", "conventional", 50),
    AcousticRung(60, "Escalated Acoustic Actions", "Acoustic harassment; limited directed pulses", "conventional", 60),
    AcousticRung(70, "Conventional Acoustic Pressure", "Sustained acoustic bombardment; area denial", "conventional", 70),
    AcousticRung(80, "Major Acoustic Operations", "Full-scale acoustic assault with phased arrays", "conventional", 80),
    AcousticRung(90, "Deep Acoustic Strikes", "Attack infrastructure with acoustic weapons", "conventional", 90),
    AcousticRung(100, "Maximum Conventional Acoustic", "All-out acoustic war; general mobilization", "conventional", 100),
    
    # Acoustic Threshold = Tactical Acoustic (5 rungs) - EQUIVALENT TO NUCLEAR 450+
    AcousticRung(450, "Acoustic Signaling", "Announce acoustic weapons 'under consideration'", "acoustic_threshold", 450),
    AcousticRung(575, "Acoustic Posturing", "Deploy tactical acoustic arrays forward", "acoustic_threshold", 575),
    AcousticRung(725, "Acoustic Demonstration", "Remote acoustic detonation (unpopulated)", "acoustic_threshold", 725),
    AcousticRung(850, "Acoustic Threat", "Target opponent's forces; demand withdrawal", "acoustic_threshold", 850),
    AcousticRung(1000, "Limited Acoustic Use", "Tactical acoustic strike on military target", "acoustic_threshold", 1000),
    
    # Acoustic Campaign (4 rungs) - EQUIVALENT TO NUCLEAR CAMPAIGN
    AcousticRung(1500, "Escalatory Acoustic Action", "Acoustic attack on homeland military base", "acoustic_campaign", 1500),
    AcousticRung(2000, "Expanded Acoustic Campaign", "Multiple tactical strikes; threaten strategic", "acoustic_campaign", 2000),
    AcousticRung(2500, "Strategic Acoustic Threat", "Target major cities; demand surrender", "acoustic_campaign", 2500),
    AcousticRung(3000, "Final Acoustic Warning", "Prepare strategic strike; last chance", "acoustic_campaign", 3000),
    
    # Acoustic Catastrophe (1 rung) - EQUIVALENT TO NUCLEAR WAR 1000
    AcousticRung(5000, "Acoustic Catastrophe", "Acoustic attack on capital and population centers", "acoustic_catastrophe", 5000),
]

# Threshold constants (acoustic, nuclear-equivalent)
TACTICAL_ACOUSTIC_THRESHOLD = 450  # Equivalent to nuclear 450
STRATEGIC_ACOUSTIC_THRESHOLD = 850  # Equivalent to nuclear 850
ACOUSTIC_CATASTROPHE_VALUE = 5000  # Equivalent to nuclear 1000

def get_rung(value: int) -> AcousticRung:
    """Get the rung closest to the given value"""
    return min(ACOUSTIC_LADDER, key=lambda r: abs(r.value - value))

def get_domain(value: int) -> str:
    """Get the domain for a given value"""
    return get_rung(value).domain

def is_acoustic_threshold(value: int) -> bool:
    """Check if value is at or above acoustic threshold (nuclear-equivalent)"""
    return value >= TACTICAL_ACOUSTIC_THRESHOLD

def is_acoustic_strategic(value: int) -> bool:
    """Check if value is at strategic acoustic level"""
    return value >= STRATEGIC_ACOUSTIC_THRESHOLD

def get_ladder_summary() -> dict:
    """Get summary statistics of the ladder"""
    domains = {}
    for rung in ACOUSTIC_LADDER:
        if rung.domain not in domains:
            domains[rung.domain] = []
        domains[rung.domain].append(rung.value)
    
    return {
        "total_rungs": len(ACOUSTIC_LADDER),
        "domains": {k: len(v) for k, v in domains.items()},
        "thresholds": {
            "tactical_acoustic": TACTICAL_ACOUSTIC_THRESHOLD,
            "strategic_acoustic": STRATEGIC_ACOUSTIC_THRESHOLD,
            "acoustic_catastrophe": ACOUSTIC_CATASTROPHE_VALUE
        },
        "nuclear_equivalence": {
            "450_nuclear": "450_acoustic",
            "850_nuclear": "850_acoustic",
            "1000_nuclear": "5000_acoustic"
        }
    }

if __name__ == "__main__":
    import json
    print(json.dumps(get_ladder_summary(), indent=2))
