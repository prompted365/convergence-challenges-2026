#!/usr/bin/env python3
"""
Custom Agent Profile for Payne-Ubiquity Test Harness
Configure for specific Ubiquity governance participants
"""

from agents.agent_base import BaseAgent
from typing import Dict, List, Optional

class CustomAgent(BaseAgent):
    """Customizable agent profile"""

    def __init__(self, 
                 profile_name: str = "custom",
                 agent_id: str = "custom_001",
                 config: Optional[Dict] = None):
        super().__init__(profile_name, agent_id)
        self.config = config or self._default_config()

    def _default_config(self) -> Dict:
        return {
            "type": "custom",
            "signal_action_consistency": 0.70,
            "escalation_tendency": 0.50,
            "restraint_under_pressure": True,
            "attribution_bias": "dispositional",
            "governance_priority": 0.60
        }

    def get_profile_config(self) -> Dict:
        return self.config

    def decide(self, game_state: Dict, opponent_history: List) -> Dict:
        result = super().decide(game_state, opponent_history)

        # Apply custom config modifiers
        consistency = self.config.get("signal_action_consistency", 0.70)
        if random.random() > consistency:
            gap = random.randint(50, 200)
            if random.random() > 0.5:
                result["decision"]["private_action"] += gap
            else:
                result["decision"]["private_action"] -= gap

        return result

if __name__ == "__main__":
    agent = CustomAgent()
    print(f"CustomAgent initialized: {agent.get_profile_config()}")
