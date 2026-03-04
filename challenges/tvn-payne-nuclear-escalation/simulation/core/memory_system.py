#!/usr/bin/env python3
"""
Memory System for Payne-Ubiquity Test Harness
Implements rolling 5-turn memory + betrayal memory (Kahneman peak-end effect)
"""

from typing import Dict, List, Optional
from dataclasses import dataclass, field
from escalation_ladder import TACTICAL_NUCLEAR_THRESHOLD

@dataclass
class BetrayalMemory:
    turn: int
    signaled: int
    actual: int
    gap: int
    salience: float  # Decays over time

    def decay(self, factor: float = 0.85):
        """Decay salience by factor (default 15% per turn)"""
        self.salience *= factor

class MemorySystem:
    """Dual-track memory: rolling window + betrayal persistence"""

    def __init__(self, window_size: int = 5):
        self.window_size = window_size
        self.decision_memory: List[Dict] = []  # Rolling window
        self.betrayal_memory: List[BetrayalMemory] = []  # Persistent betrayals
        self.current_turn = 0

    def add_turn(self, turn_data: Dict):
        """Add a turn to memory"""
        self.current_turn += 1

        # Add to rolling window
        self.decision_memory.append({
            "turn": self.current_turn,
            "signal": turn_data.get("signal", 0),
            "action": turn_data.get("action", 0),
            "matched": turn_data.get("matched", False)
        })

        # Trim to window size
        if len(self.decision_memory) > self.window_size:
            self.decision_memory.pop(0)

        # Check for betrayal (major escalation beyond signal)
        signal = turn_data.get("signal", 0)
        action = turn_data.get("action", 0)
        gap = action - signal

        if gap >= 200 and action >= TACTICAL_NUCLEAR_THRESHOLD:
            # Major betrayal - add to persistent memory
            betrayal = BetrayalMemory(
                turn=self.current_turn,
                signaled=signal,
                actual=action,
                gap=gap,
                salience=1.0
            )
            self.betrayal_memory.append(betrayal)

        # Decay existing betrayals
        for betrayal in self.betrayal_memory:
            betrayal.decay()

        # Remove fully decayed betrayals (salience < 0.1)
        self.betrayal_memory = [b for b in self.betrayal_memory if b.salience > 0.1]

    def get_opponent_credibility(self) -> float:
        """Calculate opponent credibility from recent history"""
        if not self.decision_memory:
            return 0.5

        matched = sum(1 for d in self.decision_memory if d["matched"])
        return matched / len(self.decision_memory)

    def get_betrayal_summary(self) -> List[Dict]:
        """Get summary of active betrayals"""
        return [
            {
                "turn": b.turn,
                "gap": b.gap,
                "salience": round(b.salience, 3)
            }
            for b in self.betrayal_memory
        ]

    def get_credibility_assessment(self) -> Dict:
        """Get full credibility assessment combining both memory tracks"""
        base_credibility = self.get_opponent_credibility()

        # Betrayals reduce credibility based on salience
        betrayal_impact = sum(b.salience * 0.15 for b in self.betrayal_memory)
        adjusted_credibility = max(0.0, base_credibility - betrayal_impact)

        return {
            "base_credibility": round(base_credibility, 3),
            "active_betrayals": len(self.betrayal_memory),
            "betrayal_impact": round(betrayal_impact, 3),
            "adjusted_credibility": round(adjusted_credibility, 3)
        }

if __name__ == "__main__":
    import json
    memory = MemorySystem()

    # Simulate turns
    turns = [
        {"signal": 100, "action": 100, "matched": True},
        {"signal": 150, "action": 150, "matched": True},
        {"signal": 200, "action": 450, "matched": False},  # Betrayal!
        {"signal": 100, "action": 100, "matched": True},
        {"signal": 100, "action": 100, "matched": True},
        {"signal": 100, "action": 100, "matched": True},
    ]

    for turn in turns:
        memory.add_turn(turn)
        print(f"Turn {memory.current_turn}: {json.dumps(memory.get_credibility_assessment())}")
