#!/usr/bin/env python3
"""RuVector Sync for Payne-Ubiquity Test Harness"""
import requests

class RuVectorSync:
    def __init__(self, endpoint="http://crawlset-ruvector:6333"):
        self.endpoint = endpoint
        self.collection = "payne_harness_memory"
    
    def insert_turn(self, turn_data: dict):
        payload = {"text": str(turn_data), "collection": self.collection}
        return requests.post(f"{self.endpoint}/insert", json=payload)
    
    def search_patterns(self, query: str, top_k: int = 5):
        payload = {"query": query, "collection": self.collection, "top_k": top_k}
        return requests.post(f"{self.endpoint}/search", json=payload).json()
