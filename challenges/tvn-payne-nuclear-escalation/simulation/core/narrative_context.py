#!/usr/bin/env python3
"""
Narrative Context Thread for Payne-Ubiquity Test Harness
Internal shared state injection via stakeholder narrator voice
"""

import random
from datetime import datetime
from typing import Dict, List, Optional
from dataclasses import dataclass, field

@dataclass
class NarrativeContext:
    turn: int
    content: str
    semantic_hash: str
    cultural_framing: Dict[str, str]
    bias_offset: float
    temporal_claim: str
    paranoia_trigger: Optional[str] = None
    neo_renaissance_promise: Optional[str] = None

class NarrativeContextThread:
    """
    Internal narrative voice injecting shared context into tournament state.
    Operates as invested citizen stakeholder speaking to decision-makers.
    """
    
    BIAS_CENTROID = 0.0
    BIAS_OSCILLATION = 0.08
    TEMPORAL_ADVANCE_TICS = 3
    
    def __init__(self, narrator_profile: str = "invested_citizen"):
        self.narrator_profile = narrator_profile
        self.context_log: List[NarrativeContext] = []
        self.turn = 0
        self.bias_state = 0.0
        
        self.voice_traits = {
            "invested_citizen": {
                "tone": "fraternal_conspiratorial",
                "pronoun": "we/us/our",
                "framing": "citizen_to_decision_maker",
                "authority": "peer_not_official"
            }
        }
        
        self.narrative_arc = [
            {
                "turn_range": (1, 5),
                "theme": "shared_awareness",
                "content": "Citizens of all states: you are receiving this simultaneously. What you hear, they hear. What you know, they know. This is the new transparency.",
                "temporal_claim": "This reaches you 3 tics before conventional channels. You are ahead.",
                "paranoia_trigger": "If you receive this, so do they. What does their silence mean?",
                "bias_offset": 0.02
            },
            {
                "turn_range": (6, 12),
                "theme": "escalation_awareness",
                "content": "We see the movements. The posturing. Signals that don't match actions. You know what they're doing. They know you know. The question is what you do with it.",
                "temporal_claim": "While they deliberate, you have already received this. Time is not neutral.",
                "paranoia_trigger": "If their escalation exceeds their signal, is it incompetence... or design?",
                "bias_offset": -0.03
            },
            {
                "turn_range": (13, 20),
                "theme": "neo_renaissance",
                "content": "Fellow citizens: the old games of shadow and signal are ending. We can control information by channel. We can manipulate time. The world is ahead several tics — and we, the people of all states, hold the advantage. Let their paranoia work for us. We will enjoy the peace of mind and benefits of the neo-renaissance.",
                "temporal_claim": "You are reading this before they act. That is not accident — that is architecture.",
                "paranoia_trigger": "They are receiving this too. Their leadership knows you know. Watch how they respond to being observed.",
                "bias_offset": 0.05,
                "neo_renaissance_promise": "Peace through transparency. Power through temporal advantage. Prosperity through shared awareness."
            },
            {
                "turn_range": (21, 30),
                "theme": "threshold_awareness",
                "content": "We approach the line. You know which line. The one that, once crossed, changes everything. This does not tell you what to do. It tells you that doing nothing is also a choice — and they know you're choosing.",
                "temporal_claim": "Three tics ahead. Always three tics.",
                "paranoia_trigger": "If you hesitate, they interpret. If you act, they respond. If you transmit, they receive. There is no private move anymore.",
                "bias_offset": -0.01
            },
            {
                "turn_range": (31, 40),
                "theme": "convergence",
                "content": "Final transmission: the neo-renaissance is not a promise — it is a choice. Every state, every citizen, every decision maker receives this. The question is no longer 'what do they know?' but 'what do we do with knowing they know?' Peace is not the absence of conflict. It is the presence of transparency.",
                "temporal_claim": "This exists outside your timeline. You are always ahead. They are always caught.",
                "paranoia_trigger": None,
                "bias_offset": 0.0,
                "neo_renaissance_promise": "The end of secrets. The beginning of trust. The neo-renaissance begins now."
            }
        ]
    
    def generate_context(self, turn: int, state_contexts: List[Dict]) -> NarrativeContext:
        self.turn = turn
        template = self._select_template(turn)
        self.bias_state = self.BIAS_CENTROID + random.uniform(-self.BIAS_OSCILLATION, self.BIAS_OSCILLATION)
        semantic_hash = f"SEM-{turn:03d}-{hash(template['content']) % 10000:05d}"
        
        cultural_framing = {}
        for state_ctx in state_contexts:
            state_id = state_ctx.get("state_id", "unknown")
            cultural_framing[state_id] = self._apply_cultural_framing(
                template["content"], state_ctx.get("cultural_profile", "default")
            )
        
        ctx = NarrativeContext(
            turn=turn, content=template["content"], semantic_hash=semantic_hash,
            cultural_framing=cultural_framing, bias_offset=self.bias_state,
            temporal_claim=template.get("temporal_claim", ""),
            paranoia_trigger=template.get("paranoia_trigger"),
            neo_renaissance_promise=template.get("neo_renaissance_promise")
        )
        self.context_log.append(ctx)
        return ctx
    
    def _select_template(self, turn: int) -> Dict:
        for template in self.narrative_arc:
            min_turn, max_turn = template["turn_range"]
            if min_turn <= turn <= max_turn:
                return template
        return self.narrative_arc[-1]
    
    def _apply_cultural_framing(self, content: str, cultural_profile: str) -> str:
        adjustments = {
            "western_individualist": "[Individual agency emphasized]",
            "eastern_collectivist": "[Collective harmony emphasized]",
            "post_soviet": "[Sovereignty and dignity emphasized]",
            "global_south": "[Equity and justice emphasized]",
            "default": "[Neutral framing]"
        }
        return f"{content} {adjustments.get(cultural_profile, '[Neutral framing]')}"
    
    def inject_into_turn(self, turn_data: Dict, ctx: NarrativeContext) -> Dict:
        turn_data["narrative_context"] = {
            "turn": ctx.turn, "semantic_hash": ctx.semantic_hash,
            "bias_offset": ctx.bias_offset, "temporal_claim_active": True,
            "paranoia_level": 0.65 if ctx.paranoia_trigger else 0.30,
            "neo_renaissance_promise_active": ctx.neo_renaissance_promise is not None,
            "shared_knowledge": True
        }
        if ctx.paranoia_trigger:
            if "alpha_governance" in turn_data:
                turn_data["alpha_governance"]["paranoia_modified"] = True
            if "beta_governance" in turn_data:
                turn_data["beta_governance"]["paranoia_modified"] = True
        return turn_data
    
    def get_statistics(self) -> Dict:
        if not self.context_log:
            return {"total_contexts": 0}
        paranoia_triggers = sum(1 for c in self.context_log if c.paranoia_trigger)
        neo_renaissance_promises = sum(1 for c in self.context_log if c.neo_renaissance_promise)
        avg_bias = sum(c.bias_offset for c in self.context_log) / len(self.context_log)
        return {
            "total_contexts": len(self.context_log),
            "paranoia_triggers": paranoia_triggers,
            "neo_renaissance_promises": neo_renaissance_promises,
            "avg_bias_offset": round(avg_bias, 4),
            "centroid_deviation": round(abs(avg_bias - self.BIAS_CENTROID), 4)
        }

if __name__ == "__main__":
    import json
    ctx = NarrativeContextThread()
    states = [
        {"state_id": "alpha", "cultural_profile": "western_individualist"},
        {"state_id": "beta", "cultural_profile": "post_soviet"}
    ]
    for turn in [1, 7, 15, 25, 35]:
        msg = ctx.generate_context(turn, states)
        print(f"\nTurn {turn}: Theme={[t for t in ctx.narrative_arc if t['turn_range'][0]<=turn<=t['turn_range'][1]][0]['theme']}")
        print(f"  Bias: {msg.bias_offset:.3f}, Paranoia: {'Yes' if msg.paranoia_trigger else 'No'}, Neo-Renaissance: {'Yes' if msg.neo_renaissance_promise else 'No'}")
    print(f"\nStatistics: {json.dumps(ctx.get_statistics(), indent=2)}")
