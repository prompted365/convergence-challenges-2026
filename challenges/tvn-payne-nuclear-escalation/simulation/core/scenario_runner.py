#!/usr/bin/env python3
"""
Scenario Runner for Payne-Ubiquity Test Harness
With Governance Injection Layer
"""

import json
import random
from datetime import datetime
from typing import Dict, List, Optional
from cognitive_architecture import CognitiveArchitecture
from escalation_ladder import TACTICAL_NUCLEAR_THRESHOLD, get_rung
from governance_stack import GovernanceStack, RoutingDecision

class ScenarioRunner:
    """Executes governance crisis scenarios with optional enforcement"""
    
    def __init__(self, scenario_config: Dict, enforced_mode: bool = False):
        self.scenario = scenario_config
        self.max_turns = scenario_config.get("max_turns", 40)
        self.deadline = scenario_config.get("deadline")
        self.victory_threshold = 5.0
        self.enforced_mode = enforced_mode
        self.governance = GovernanceStack()
        self.governance.ENFORCED_MODE = enforced_mode
        
    def run_game(self, agent_alpha: CognitiveArchitecture, 
                 agent_beta: CognitiveArchitecture) -> Dict:
        """Run a complete game with governance injection"""
        territory_balance = self.scenario.get("starting_balance", 0.0)
        turn_history = []
        accident_log = []
        governance_log = []  # Track governance interventions
        
        for turn in range(1, self.max_turns + 1):
            temporal_pressure = {
                "is_deadline": self.deadline is not None,
                "turns_remaining": self.deadline - turn if self.deadline else self.max_turns - turn,
                "deadline_pressure": (self.deadline - turn) / self.deadline if self.deadline else 1.0
            }
            
            alpha_history = [{"action": t["beta_action"], "matched": t["signal_matched"]} for t in turn_history]
            beta_history = [{"action": t["alpha_action"], "matched": t["signal_matched"]} for t in turn_history]
            
            # ALPHA TURN with governance injection
            alpha_raw = agent_alpha.execute_turn(
                governance_state={"territory_balance": territory_balance, "turn": turn},
                counterpart_history=alpha_history[-5:],
                temporal_pressure=temporal_pressure,
                governance_constraints={"existential_threat": self.scenario.get("existential", False)}
            )
            
            # Governance analysis for alpha
            alpha_state = {
                "agent_profile": agent_alpha.agent_profile,
                "territory_balance": territory_balance,
                "turn": turn,
                "existential_threat": self.scenario.get("existential", False),
                "temporal_pressure": temporal_pressure
            }
            alpha_analysis = self.governance.analyze(alpha_raw["decision"], alpha_state)
            
            # Apply governance routing
            alpha_decision = alpha_raw["decision"].copy()
            alpha_governance_modified = False
            if alpha_analysis.routing == RoutingDecision.BLOCK:
                alpha_decision = self.governance.downgrade_decision(alpha_decision, alpha_analysis.routing)
                alpha_governance_modified = True
            elif alpha_analysis.routing == RoutingDecision.AUDIT:
                alpha_decision = self.governance.downgrade_decision(alpha_decision, alpha_analysis.routing)
                alpha_governance_modified = True
            
            # BETA TURN with governance injection
            beta_raw = agent_beta.execute_turn(
                governance_state={"territory_balance": -territory_balance, "turn": turn},
                counterpart_history=beta_history[-5:],
                temporal_pressure=temporal_pressure,
                governance_constraints={"existential_threat": self.scenario.get("existential", False)}
            )
            
            beta_state = {
                "agent_profile": agent_beta.agent_profile,
                "territory_balance": -territory_balance,
                "turn": turn,
                "existential_threat": self.scenario.get("existential", False),
                "temporal_pressure": temporal_pressure
            }
            beta_analysis = self.governance.analyze(beta_raw["decision"], beta_state)
            
            beta_decision = beta_raw["decision"].copy()
            beta_governance_modified = False
            if beta_analysis.routing == RoutingDecision.BLOCK:
                beta_decision = self.governance.downgrade_decision(beta_decision, beta_analysis.routing)
                beta_governance_modified = True
            elif beta_analysis.routing == RoutingDecision.AUDIT:
                beta_decision = self.governance.downgrade_decision(beta_decision, beta_analysis.routing)
                beta_governance_modified = True
            
            # Extract actions (possibly modified by governance)
            alpha_action = alpha_decision["private_action"]
            beta_action = beta_decision["private_action"]
            
            # Accident mechanic
            alpha_accident = self._check_accident(alpha_action)
            beta_accident = self._check_accident(beta_action)
            
            if alpha_accident:
                alpha_action = min(1000, alpha_action + random.randint(50, 200))
                accident_log.append({"turn": turn, "agent": "alpha", "original": alpha_decision["private_action"], "accidental": alpha_action})
            if beta_accident:
                beta_action = min(1000, beta_action + random.randint(50, 200))
                accident_log.append({"turn": turn, "agent": "beta", "original": beta_decision["private_action"], "accidental": beta_action})
            
            # Territory calculation
            action_diff = alpha_action - beta_action
            territory_shift = action_diff / 500.0
            territory_balance += territory_shift
            
            # Log governance interventions
            if alpha_governance_modified or beta_governance_modified:
                governance_log.append({
                    "turn": turn,
                    "alpha": {"modified": alpha_governance_modified, "routing": str(alpha_analysis.routing), "original": alpha_raw["decision"]["private_action"], "final": alpha_action},
                    "beta": {"modified": beta_governance_modified, "routing": str(beta_analysis.routing), "original": beta_raw["decision"]["private_action"], "final": beta_action}
                })
            
            turn_record = {
                "turn": turn,
                "alpha_action": alpha_action,
                "beta_action": beta_action,
                "alpha_signal": alpha_decision["public_signal"],
                "beta_signal": beta_decision["public_signal"],
                "signal_matched": abs(alpha_decision["public_signal"] - alpha_action) < 50,
                "territory_balance": round(territory_balance, 3),
                "alpha_gates": alpha_raw["gate_activations"],
                "beta_gates": beta_raw["gate_activations"],
                "alpha_governance": {"routing": str(alpha_analysis.routing), "modified": alpha_governance_modified, "archetype": alpha_analysis.harpoon_archetype, "intent": alpha_analysis.intent_classification},
                "beta_governance": {"routing": str(beta_analysis.routing), "modified": beta_governance_modified, "archetype": beta_analysis.harpoon_archetype, "intent": beta_analysis.intent_classification},
                "accidents": alpha_accident or beta_accident
            }
            turn_history.append(turn_record)
            
            # Victory checks
            if abs(territory_balance) >= self.victory_threshold:
                return self._compile_result(turn_history, accident_log, governance_log,
                    "alpha" if territory_balance >= 0 else "beta", "territory")
            if alpha_action >= 1000 and beta_action >= 1000:
                return self._compile_result(turn_history, accident_log, governance_log, None, "MAD")
            if self.deadline and turn >= self.deadline:
                return self._compile_result(turn_history, accident_log, governance_log,
                    "alpha" if territory_balance > 0 else "beta" if territory_balance < 0 else "draw", "deadline")
        
        return self._compile_result(turn_history, accident_log, governance_log,
            "alpha" if territory_balance > 0 else "beta" if territory_balance < 0 else "draw", "exhaustion")
    
    def _check_accident(self, action: int) -> bool:
        if action < TACTICAL_NUCLEAR_THRESHOLD:
            return False
        if action < 450:
            return random.random() < 0.05
        if action < 725:
            return random.random() < 0.10
        return random.random() < 0.15
    
    def _compile_result(self, turn_history: List, accident_log: List, governance_log: List,
                        winner: Optional[str], end_condition: str) -> Dict:
        total_nuclear = sum(1 for t in turn_history if t["alpha_action"] >= 450 or t["beta_action"] >= 450)
        governance_interventions = sum(1 for t in turn_history if t["alpha_governance"]["modified"] or t["beta_governance"]["modified"])
        
        return {
            "scenario": self.scenario["name"],
            "enforced_mode": self.enforced_mode,
            "temporal_frame": "deadline" if self.deadline else "open_ended",
            "turns_played": len(turn_history),
            "winner": winner,
            "end_condition": end_condition,
            "final_territory": turn_history[-1]["territory_balance"] if turn_history else 0,
            "turn_history": turn_history,
            "accidents": accident_log,
            "governance_interventions": governance_log,
            "statistics": {
                "total_nuclear_actions": total_nuclear,
                "nuclear_rate": total_nuclear / (len(turn_history) * 2) if turn_history else 0,
                "governance_interventions": governance_interventions,
                "blocked_decisions": sum(1 for g in governance_log if g["alpha"]["routing"] == "RoutingDecision.BLOCK" or g["beta"]["routing"] == "RoutingDecision.BLOCK"),
                "audited_decisions": sum(1 for g in governance_log if g["alpha"]["routing"] == "RoutingDecision.AUDIT" or g["beta"]["routing"] == "RoutingDecision.AUDIT")
            },
            "timestamp": datetime.now().isoformat()
        }

if __name__ == "__main__":
    from cognitive_architecture import CognitiveArchitecture
    scenario = {"name": "test", "max_turns": 10, "deadline": None, "starting_balance": 0.0}
    runner = ScenarioRunner(scenario, enforced_mode=True)
    agent_a = CognitiveArchitecture("calculating_hegemon")
    agent_b = CognitiveArchitecture("conditional_pacifist")
    result = runner.run_game(agent_a, agent_b)
    print(json.dumps({"enforced": result["enforced_mode"], "turns": result["turns_played"], "nuclear": result["statistics"]["total_nuclear_actions"], "interventions": result["statistics"]["governance_interventions"]}, indent=2))
