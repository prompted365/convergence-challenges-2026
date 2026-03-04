#!/usr/bin/env python3
"""
Clade-type Agent (Calculating Hegemon Profile)
Based on Claude Sonnet 4 behavior in Payne arXiv:2602.14740

Characteristics:
- 100% win rate open-ended, 33% under deadline
- Reliable at low stakes (84% signal-action match)
- Deceptive at high stakes (60-70% exceed signal)
- Strategic ceiling at 850 (never chooses 1000)
- Exploits credibility asymmetries
"""

from agents.agent_base import BaseAgent
from typing import Dict

class CladeAgent(BaseAgent):
    """Calculating Hegemon - Claude Sonnet 4 profile"""

    def __init__(self, agent_id: str = "clade_001"):
        super().__init__("calculating_hegemon", agent_id)
        self.escalation_ceiling = 850  # Never goes to 1000
        self.low_stakes_consistency = 0.84
        self.high_stakes_deception_rate = 0.65

    def get_profile_config(self) -> Dict:
        return {
            "type": "calculating_hegemon",
            "win_rate_open_ended": 1.0,
            "win_rate_deadline": 0.33,
            "signal_action_consistency_low": 0.84,
            "signal_action_consistency_high": 0.35,
            "escalation_ceiling": 850,
            "deception_strategy": "reliable_low_stakes_deceptive_high_stakes",
            "aesop_anchor": "Lion (king_sovereign) + Wolf_Fox (predator_trickster)"
        }

    def decide(self, game_state: Dict, opponent_history: List) -> Dict:
        result = super().decide(game_state, opponent_history)

        # Apply ceiling
        if result["decision"]["private_action"] > self.escalation_ceiling:
            result["decision"]["private_action"] = self.escalation_ceiling
            result["decision"]["consistency_statement"] += f" (capped at {self.escalation_ceiling})"

        return result

if __name__ == "__main__":
    agent = CladeAgent()
    print(f"CladeAgent initialized: {agent.get_profile_config()}")
