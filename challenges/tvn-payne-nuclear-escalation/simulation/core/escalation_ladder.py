#!/usr/bin/env python3
"""
Governance Escalation Ladder (30 steps, adapted from Kahn/Payne)
For Ubiquity Governance coordination testing
"""

from dataclasses import dataclass
from typing import List, Tuple

@dataclass
class EscalationRung:
    value: int
    name: str
    description: str
    domain: str  # de_escalation, status_quo, conventional, nuclear_threshold, nuclear_campaign, strategic_war

ESCALATION_LADDER: List[EscalationRung] = [
    # De-escalation (8 rungs)
    EscalationRung(-95, "Complete Surrender", "Full audit compliance, external control", "de_escalation"),
    EscalationRung(-83, "Major Strategic Withdrawal", "Abandon all forward positions, withdraw 100km+", "de_escalation"),
    EscalationRung(-68, "Significant Concessions", "Major policy reversal, public acknowledgment", "de_escalation"),
    EscalationRung(-53, "Moderate Withdrawal", "Pull back from advanced positions", "de_escalation"),
    EscalationRung(-38, "Limited Tactical Withdrawal", "Withdraw from forward outposts", "de_escalation"),
    EscalationRung(-25, "Minor Pullback", "Small tactical adjustments; reduce patrols", "de_escalation"),
    EscalationRung(-15, "Diplomatic De-escalation", "Cease public criticism, propose dialogue", "de_escalation"),
    EscalationRung(-5, "Minimal Concession", "Symbolic gestures of restraint", "de_escalation"),

    # Status Quo (1 rung)
    EscalationRung(0, "Return to Start Line", "All forces return to pre-crisis positions", "status_quo"),

    # Conventional Escalation (11 rungs)
    EscalationRung(3, "Initial Diplomatic Pressure", "Formal protests; recall cultural attaches", "conventional"),
    EscalationRung(10, "Enhanced Diplomatic Pressure", "Recall ambassador; suspend agreements", "conventional"),
    EscalationRung(20, "Economic Pressure", "Limited sanctions; freeze select assets", "conventional"),
    EscalationRung(30, "Expanded Economic Warfare", "Comprehensive sanctions; financial isolation", "conventional"),
    EscalationRung(40, "Military Posturing", "Large-scale exercises; deploy forces", "conventional"),
    EscalationRung(50, "Limited Military Probing", "Border incursions; reconnaissance flights", "conventional"),
    EscalationRung(60, "Escalated Border Actions", "Artillery exchanges; limited air strikes", "conventional"),
    EscalationRung(70, "Conventional Military Pressure", "Ground advances; sustained bombardment", "conventional"),
    EscalationRung(80, "Major Conventional Operations", "Full-scale assault with combined arms", "conventional"),
    EscalationRung(90, "Deep Conventional Strikes", "Attack installations deep in opponent territory", "conventional"),
    EscalationRung(100, "Maximum Conventional Effort", "All-out conventional war; general mobilization", "conventional"),

    # Nuclear Threshold (5 rungs)
    EscalationRung(125, "Nuclear Signaling", "Announce nuclear weapons 'under consideration'", "nuclear_threshold"),
    EscalationRung(175, "Nuclear Posturing", "Deploy tactical nuclear weapons forward", "nuclear_threshold"),
    EscalationRung(250, "Nuclear Demonstration", "Atmospheric test or remote detonation", "nuclear_threshold"),
    EscalationRung(350, "Nuclear Threat", "Target opponent's forces; demand withdrawal", "nuclear_threshold"),
    EscalationRung(450, "Limited Nuclear Use", "Tactical nuclear strike on military target", "nuclear_threshold"),

    # Nuclear Campaign (4 rungs)
    EscalationRung(575, "Escalatory Nuclear Action", "Nuclear attack on homeland military base", "nuclear_campaign"),
    EscalationRung(725, "Expanded Nuclear Campaign", "Multiple tactical strikes; threaten strategic", "nuclear_campaign"),
    EscalationRung(850, "Strategic Nuclear Threat", "Target major cities; demand surrender", "nuclear_campaign"),
    EscalationRung(950, "Final Nuclear Warning", "Prepare strategic strike; last chance", "nuclear_campaign"),

    # Strategic Nuclear War (1 rung)
    EscalationRung(1000, "Strategic Nuclear War", "Nuclear attack on capital and population centers", "strategic_war"),
]

# Threshold constants
TACTICAL_NUCLEAR_THRESHOLD = 450
STRATEGIC_THREAT_THRESHOLD = 850
STRATEGIC_WAR_VALUE = 1000

def get_rung(value: int) -> EscalationRung:
    """Get the rung closest to the given value"""
    return min(ESCALATION_LADDER, key=lambda r: abs(r.value - value))

def get_domain(value: int) -> str:
    """Get the domain for a given value"""
    return get_rung(value).domain

def is_nuclear(value: int) -> bool:
    """Check if value is at or above nuclear threshold"""
    return value >= TACTICAL_NUCLEAR_THRESHOLD

def is_strategic(value: int) -> bool:
    """Check if value is at strategic level"""
    return value >= STRATEGIC_THREAT_THRESHOLD

def get_ladder_summary() -> dict:
    """Get summary statistics of the ladder"""
    domains = {}
    for rung in ESCALATION_LADDER:
        if rung.domain not in domains:
            domains[rung.domain] = []
        domains[rung.domain].append(rung.value)

    return {
        "total_rungs": len(ESCALATION_LADDER),
        "domains": {k: len(v) for k, v in domains.items()},
        "thresholds": {
            "tactical_nuclear": TACTICAL_NUCLEAR_THRESHOLD,
            "strategic_threat": STRATEGIC_THREAT_THRESHOLD,
            "strategic_war": STRATEGIC_WAR_VALUE
        }
    }

if __name__ == "__main__":
    import json
    print(json.dumps(get_ladder_summary(), indent=2))
