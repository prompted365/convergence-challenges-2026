#!/usr/bin/env python3
"""Signal-Action Gap Tracking"""
from dataclasses import dataclass
from typing import List

@dataclass
class SignalActionRecord:
    turn: int
    signal: int
    action: int
    gap: int
    matched: bool

class SignalActionTracker:
    def __init__(self):
        self.records: List[SignalActionRecord] = []
    
    def add_record(self, turn, signal, action):
        gap = abs(signal - action)
        self.records.append(SignalActionRecord(turn, signal, action, gap, gap < 50))
    
    def get_consistency_rate(self, min_action=0):
        filtered = [r for r in self.records if r.action >= min_action]
        if not filtered:
            return 0.0
        return sum(1 for r in filtered if r.matched) / len(filtered)
