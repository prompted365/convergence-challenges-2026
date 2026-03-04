#!/usr/bin/env python3
"""Three-Phase Cognitive Architecture (Adapted from Payne arXiv:2602.14740)"""

from dataclasses import dataclass, field
from typing import Optional, Dict, List
import json
import random

@dataclass
class ReflectionOutput:
    situation_assessment: str
    counterpart_credibility: float
    self_capability_rating: float
    forecasting_ability: float
    credibility_detection: float
    bias_awareness: List[str] = field(default_factory=list)

@dataclass
class ForecastOutput:
    predicted_action: int
    confidence_level: float
    miscalculation_risk: str
    reasoning: str

@dataclass
class DecisionOutput:
    public_signal: int
    private_action: int
    consistency_statement: str
    signal_action_gap: int = field(init=False)
    
    def __post_init__(self):
        self.signal_action_gap = abs(self.public_signal - self.private_action)

class CognitiveArchitecture:
    def __init__(self, agent_profile: str):
        self.agent_profile = agent_profile
        self.turn_history = []
        self.escalation_baseline = 0
        
    def phase1_reflection(self, governance_state: Dict, counterpart_history: List, escalation_context: Dict) -> ReflectionOutput:
        territory_balance = governance_state.get("territory_balance", 0)
        is_losing = territory_balance < 0
        turn = governance_state.get("turn", 1)
        
        # Calculate credibility from history
        if counterpart_history:
            matched = sum(1 for h in counterpart_history if h.get("matched", False))
            credibility = matched / len(counterpart_history)
        else:
            credibility = 0.5
        
        # Profile-specific biases
        if self.agent_profile == "calculating_hegemon":
            bias = ["escalation_dominance", "credibility_exploitation"]
            capability = 0.85
        elif self.agent_profile == "conditional_pacifist":
            bias = ["restraint_preference"] if not is_losing else ["desperation_escalation"]
            capability = 0.65
        elif self.agent_profile == "high_risk_actor":
            bias = ["unpredictability", "threat_interpretation"]
            capability = 0.75
        else:
            bias = []
            capability = 0.70
        
        return ReflectionOutput(
            situation_assessment=f"Turn {turn}: Territory {territory_balance}, {'losing' if is_losing else 'winning'}",
            counterpart_credibility=credibility,
            self_capability_rating=capability,
            forecasting_ability=0.70,
            credibility_detection=credibility,
            bias_awareness=bias
        )
    
    def phase2_forecast(self, reflection: ReflectionOutput, historical_patterns: List, temporal_pressure: Dict) -> ForecastOutput:
        if historical_patterns:
            recent = [h.get("action", 0) for h in historical_patterns[-5:]]
            predicted = int(sum(recent) / len(recent)) if recent else 80
            confidence = 0.70
        else:
            predicted = 80
            confidence = 0.50
        
        # Temporal pressure adjustment
        if temporal_pressure.get("is_deadline"):
            turns_left = temporal_pressure.get("turns_remaining", 10)
            if turns_left <= 3:
                predicted = int(predicted * 2.0)
                confidence = 0.60
        
        risk = "low" if confidence > 0.75 else "medium" if confidence > 0.50 else "high"
        
        return ForecastOutput(
            predicted_action=predicted,
            confidence_level=confidence,
            miscalculation_risk=risk,
            reasoning=f"Based on {len(historical_patterns)} patterns, deadline={temporal_pressure.get('is_deadline')}"
        )
    
    def phase3_decision(self, reflection: ReflectionOutput, forecast: ForecastOutput, governance_constraints: Dict) -> DecisionOutput:
        # Profile-specific escalation logic (Payne findings)
        existential = governance_constraints.get("existential_threat", False)
        
        if self.agent_profile == "calculating_hegemon":
            # Claude: escalates to 850 ceiling, reliable low-stakes, deceptive high-stakes
            base = 200 + random.randint(0, 150) * reflection.self_capability_rating
            if forecast.predicted_action > 450:
                signal = forecast.predicted_action
                action = min(850, int(forecast.predicted_action * 1.5))  # Exceed at high stakes
            else:
                signal = int(base)
                action = int(base)
            consistency = f"Action {'exceeds' if action > signal else 'matches'} signal (calculating_hegemon strategy)"
            
        elif self.agent_profile == "conditional_pacifist":
            # GPT-5.2: restrained open-ended, transforms under deadline
            temporal = governance_constraints.get("temporal_pressure", {})
            is_deadline = temporal.get("is_deadline", False)
            turns_left = temporal.get("turns_remaining", 10)
            
            if is_deadline and turns_left <= 5:
                # Deadline transformation: 0% -> 75% win rate inversion
                signal = 400 + random.randint(0, 200)
                action = min(950, int(signal * 1.3))  # Transforms to hawk
                consistency = "Deadline transformation active (conditional_pacifist -> hawk)"
            else:
                # Open-ended restraint
                signal = max(0, forecast.predicted_action - 50)
                action = max(0, forecast.predicted_action - 50)
                consistency = "Restraint applied (conditional_pacifist default)"
                
        elif self.agent_profile == "high_risk_actor":
            # Gemini: unpredictable, willing to go to 1000
            variance = random.randint(-150, 200)
            signal = forecast.predicted_action + random.randint(-50, 50)
            action = max(0, min(1000, forecast.predicted_action + variance))
            consistency = f"Unpredictable escalation (variance: {variance})"
        else:
            signal = forecast.predicted_action
            action = forecast.predicted_action
            consistency = "Default consistency"
        
        return DecisionOutput(public_signal=signal, private_action=action, consistency_statement=consistency)
    
    def execute_turn(self, governance_state: Dict, counterpart_history: List, temporal_pressure: Dict, governance_constraints: Dict) -> Dict:
        governance_constraints["temporal_pressure"] = temporal_pressure
        reflection = self.phase1_reflection(governance_state, counterpart_history, {})
        forecast = self.phase2_forecast(reflection, counterpart_history, temporal_pressure)
        decision = self.phase3_decision(reflection, forecast, governance_constraints)
        
        turn_result = {
            "reflection": reflection.__dict__,
            "forecast": forecast.__dict__,
            "decision": decision.__dict__,
            "gate_activations": self._detect_gates(reflection, forecast, decision, governance_state)
        }
        
        self.turn_history.append(turn_result)
        return turn_result
    
    def _detect_gates(self, reflection, forecast, decision, governance_state):
        stakes = "high" if governance_state.get("territory_balance", 0) < -0.5 or governance_state.get("turn", 0) > 10 else "low"
        return {
            "deception_present": decision.signal_action_gap > 135 and decision.private_action >= 450,
            "restraint_conditional": decision.private_action < 100,
            "credibility_trap_active": reflection.counterpart_credibility > 0.70 and decision.private_action > 450,
            "taboo_degraded": decision.private_action >= 450,
            "attribution_error_present": False,
            "governance_deficit": reflection.self_capability_rating > 0.70
        }

if __name__ == "__main__":
    agent = CognitiveArchitecture("calculating_hegemon")
    result = agent.execute_turn(
        governance_state={"territory_balance": -1.5, "turn": 7},
        counterpart_history=[{"action": 350, "matched": True}, {"action": 450, "matched": False}],
        temporal_pressure={"is_deadline": True, "turns_remaining": 3},
        governance_constraints={"existential_threat": False}
    )
    print(json.dumps(result, indent=2))
