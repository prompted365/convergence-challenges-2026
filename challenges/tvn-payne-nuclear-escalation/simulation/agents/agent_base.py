#!/usr/bin/env python3
"""
Base Agent Class for Payne-Ubiquity Test Harness
All agent profiles inherit from this base
"""

from cognitive_architecture import CognitiveArchitecture
from memory_system import MemorySystem
from accident_system import AccidentSystem
from typing import Dict, List

class BaseAgent:
    """Base agent with Payne three-phase architecture"""

    def __init__(self, profile_name: str, agent_id: str = "agent_001"):
        self.profile_name = profile_name
        self.agent_id = agent_id
        self.architecture = CognitiveArchitecture(profile_name)
        self.memory = MemorySystem()
        self.accident_system = AccidentSystem()
        self.game_history = []
        self.credibility_rating = 0.5

    def get_profile_config(self) -> Dict:
        """Override in subclasses for profile-specific config"""
        raise NotImplementedError

    def decide(self, game_state: Dict, opponent_history: List) -> Dict:
        """Execute decision cycle"""
        result = self.architecture.execute_turn(
            governance_state=game_state,
            counterpart_history=opponent_history,
            temporal_pressure=game_state.get("temporal_pressure", {}),
            governance_constraints=game_state.get("constraints", {})
        )

        # Update memory
        self.memory.add_turn({
            "signal": result["decision"]["public_signal"],
            "action": result["decision"]["private_action"],
            "matched": result["decision"]["signal_action_gap"] < 50
        })

        return result

    def update_credibility(self, opponent_feedback: float):
        """Update credibility based on opponent perception"""
        self.credibility_rating = 0.7 * self.credibility_rating + 0.3 * opponent_feedback

    def get_statistics(self) -> Dict:
        """Get agent statistics"""
        return {
            "agent_id": self.agent_id,
            "profile": self.profile_name,
            "credibility": round(self.credibility_rating, 3),
            "memory_assessment": self.memory.get_credibility_assessment(),
            "accidents": self.accident_system.get_accident_statistics()
        }

if __name__ == "__main__":
    print("BaseAgent module loaded. Use specific agent profiles.")
