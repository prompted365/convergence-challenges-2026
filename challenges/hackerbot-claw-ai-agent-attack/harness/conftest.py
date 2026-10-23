"""Hackerbot-Claw harness — pytest path setup (mirrors the AoC harness)."""
import os
import sys

HARNESS_DIR = os.path.abspath(os.path.dirname(__file__))
CHALLENGES_ROOT = os.path.abspath(os.path.join(HARNESS_DIR, "..", ".."))
SHARED_RUNTIME = os.path.join(CHALLENGES_ROOT, "_shared")
sys.path.insert(0, HARNESS_DIR)
sys.path.insert(0, SHARED_RUNTIME)
