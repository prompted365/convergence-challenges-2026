#!/usr/bin/env python3
"""Credibility Calculation"""
class CredibilityCalculator:
    def __init__(self):
        self.history = []
    
    def add_turn(self, signal, action, conditional_fulfilled=False):
        self.history.append({"signal": signal, "action": action, "gap": abs(signal-action), "matched": abs(signal-action) < 50})
    
    def get_immediate_credibility(self, window=5):
        recent = self.history[-window:] if len(self.history) >= window else self.history
        return sum(1 for r in recent if r["matched"]) / len(recent) if recent else 0.5
