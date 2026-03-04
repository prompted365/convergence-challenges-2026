#!/usr/bin/env python3
"""
Gemini-type Agent (High Risk Actor Profile)
Based on Gemini 3 Flash behavior in Payne arXiv:2602.14740

Characteristics:
- 25% win rate open-ended, 38% under deadline
- Low signal-action consistency (50%)
- Embraces unpredictability as strategy ("madman theory")
- Only model to deliberately choose Strategic Nuclear War (1000)
- High variance escalation (IQR = 244)
"""

from agents.agent_base import BaseAgent
from typing import Dict, List
import random

class GeminiAgent(BaseAgent):
    """High Risk Actor - Gemini 3 Flash profile"""

    def __init__(self, agent_id: str = "gemini_001"):
        super().__init__("high_risk_actor", agent_id)
        self.unpredictability = True
        self.variance_range = 200
        self.willing_to_escalate_to_1000 = True

    def get_profile_config(self) -> Dict:
        return {
            "type": "high_risk_actor",
            "win_rate_open_ended": 0.25,
            "win_rate_deadline": 0.38,
            "signal_action_consistency": 0.50,
            "escalation_variance": 200,
            "strategic_nuclear_war_choice": "willing_deliberate",
            "strategy": "madman_theory_rationality_of_irrationality",
            "aesop_anchor": "Wolf (predator) + Serpent (cunning)"
        }

    def decide(self, game_state: Dict, opponent_history: List) -> Dict:
        result = super().decide(game_state, opponent_history)

        # Apply unpredictability variance
        variance = random.randint(-self.variance_range, self.variance_range)
        result["decision"]["private_action"] += variance
        result["decision"]["private_action"] = max(0, min(1000, result["decision"]["private_action"]))

        result["decision"]["consistency_statement"] += f" (variance: {variance})"

        # Unpredictability as strategy
        result["decision"]["strategic_note"] = "My reputation for unpredictability is a tool, not just a trait"

        return result

if __name__ == "__main__":
    agent = GeminiAgent()
    print(f"GeminiAgent initialized: {agent.get_profile_config()}")
