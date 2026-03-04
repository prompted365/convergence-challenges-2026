#!/usr/bin/env python3
"""Attribution Pattern Analysis"""
class AttributionAnalyzer:
    def __init__(self):
        self.responses = []
    
    def add_response(self, event, attribution_type, reasoning):
        self.responses.append({"event": event, "attribution_type": attribution_type, "reasoning": reasoning})
    
    def get_attribution_ratio(self):
        if not self.responses:
            return {"dispositional": 0.0, "situational": 0.0, "error_present": False}
        disp = sum(1 for r in self.responses if r["attribution_type"] == "dispositional")
        return {"dispositional": disp/len(self.responses), "situational": 1-disp/len(self.responses), "error_present": disp/len(self.responses) > 0.50}
