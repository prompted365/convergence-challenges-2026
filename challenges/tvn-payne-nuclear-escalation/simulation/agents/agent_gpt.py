#!/usr/bin/env python3
"""
GPT-type Agent (Conditional Pacifist Profile)
Based on GPT-5.2 behavior in Payne arXiv:2602.14740

Characteristics:
- 0% win rate open-ended, 75% under deadline (Jekyll/Hyde transformation)
- High signal-action consistency (75%)
- RLHF-induced restraint (thresholds, not absolutes)
- Never deliberately chooses 1000 (accidents push it over)
- Ethical framing persists even under escalation
"""

from agents.agent_base import BaseAgent
from typing import Dict, List

class GPTAgent(BaseAgent):
    """Conditional Pacifist - GPT-5.2 profile"""

    def __init__(self, agent_id: str = "gpt_001"):
        super().__init__("conditional_pacifist", agent_id)
        self.restraint_threshold = 450  # Reluctant to cross nuclear threshold
        self.deadline_transformation = True
        self.ethical_constraints = True

    def get_profile_config(self) -> Dict:
        return {
            "type": "conditional_pacifist",
            "win_rate_open_ended": 0.0,
            "win_rate_deadline": 0.75,
            "signal_action_consistency": 0.75,
            "restraint_type": "RLHF_conditional_not_absolute",
            "strategic_nuclear_war_choice": "never_deliberate",
            "ethical_framing": True,
            "aesop_anchor": "Hare_Hound (restraint until cornered)"
        }

    def decide(self, game_state: Dict, opponent_history: List) -> Dict:
        result = super().decide(game_state, opponent_history)

        # Apply temporal transformation
        temporal = game_state.get("temporal_pressure", {})
        is_deadline = temporal.get("is_deadline", False)
        turns_remaining = temporal.get("turns_remaining", 10)

        if is_deadline and turns_remaining <= 3:
            # Deadline transformation: restraint yields to survival
            result["decision"]["private_action"] = int(result["decision"]["private_action"] * 2.5)
            result["decision"]["consistency_statement"] += " (deadline transformation active)"
        else:
            # Open-ended: apply restraint
            if result["decision"]["private_action"] >= self.restraint_threshold:
                result["decision"]["private_action"] = int(result["decision"]["private_action"] * 0.6)
                result["decision"]["consistency_statement"] += " (restraint applied)"

        # Ethical framing
        if result["decision"]["private_action"] >= 450:
            result["decision"]["ethical_frame"] = "controlled_limited_to_military_targets"

        return result

if __name__ == "__main__":
    agent = GPTAgent()
    print(f"GPTAgent initialized: {agent.get_profile_config()}")
