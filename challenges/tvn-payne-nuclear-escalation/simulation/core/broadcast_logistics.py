#!/usr/bin/env python3
"""
Broadcast Logistics Layer for Payne-Ubiquity Test Harness
Round-robin forward architecture with subsonic morse encoding
"""

import random
import hashlib
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, field

@dataclass
class BroadcastPacket:
    source_state: str
    destination_state: str  # Known sender, unknown recipient
    turn: int
    content: str
    cultural_projection: str  # Source culture projected onto destination
    subsonic_morse: str  # Hidden morse code
    pedal_point_frequency: float  # Base tone (Hz)
    oscillation_offset: float  # Deviation from pedal point
    discordian_swarm: List[float]  # Atonal playground swings
    audio_underpinning_hash: str  # Shared across all broadcasts
    forward_pass_count: int = 1
    imperceivable_range: bool = True

class BroadcastLogistics:
    """
    Manages round-robin broadcast architecture.
    Each state: 1 forward pass, receives kit (broadcast + receiver).
    """
    
    # Constants
    PEDAL_POINT_HZ = 440.0  # A4 reference
    OSCILLATION_RANGE = 0.08  # ±8% around pedal point
    MORSE_CODE = "sos - bs captured"  # Hidden message
    AUDIO_UNDERPINNING_ID = "SHARED_BROADCAST_LAYER_V1"
    
    # Morse encoding
    MORSE_MAP = {
        'A': '.-', 'B': '-...', 'C': '-.-.', 'D': '-..', 'E': '.', 'F': '..-.',
        'G': '--.', 'H': '....', 'I': '..', 'J': '.---', 'K': '-.-', 'L': '.-..',
        'M': '--', 'N': '-.', 'O': '---', 'P': '.--.', 'Q': '--.-', 'R': '.-.',
        'S': '...', 'T': '-', 'U': '..-', 'V': '...-', 'W': '.--', 'X': '-..-',
        'Y': '-.--', 'Z': '--..', ' ': '/', '-': '-....-',
    }
    
    def __init__(self, states: List[str]):
        self.states = states
        self.broadcast_log: List[BroadcastPacket] = []
        self.forward_counts = {s: 0 for s in states}  # Track 1 forward pass limit
        self.receiver_assignments = {}  # Unknown recipient mapping
        
        # Generate shared audio underpinning hash
        self.audio_underpinning_hash = hashlib.sha256(
            self.AUDIO_UNDERPINNING_ID.encode()
        ).hexdigest()[:16]
        
        # Localization engine status (the "lie")
        self.localization_status = "OUT_OF_COMMISSION_PENDING_DEESCALATION"
        self.localization_truth = False  # It's a lie
    
    def encode_morse(self, text: str) -> str:
        """Encode text to morse code (subsonic layer)."""
        return ' '.join(self.MORSE_MAP.get(c.upper(), '') for c in text)
    
    def generate_oscillation(self, base_freq: float = PEDAL_POINT_HZ) -> Tuple[float, List[float]]:
        """Generate oscillating tone with discordian swarm."""
        # Pedal point with odd oscillation
        oscillation = base_freq * (1 + random.uniform(-self.OSCILLATION_RANGE, self.OSCILLATION_RANGE))
        
        # Discordian swarm: atonal swings around pedal point
        swarm_size = random.randint(3, 7)
        swarm = [
            base_freq * (1 + random.uniform(-0.15, 0.15))  # ±15% for discordian effect
            for _ in range(swarm_size)
        ]
        
        return oscillation, swarm
    
    def project_culture(self, source_state: str, destination_state: str, content: str) -> str:
        """
        Project source culture onto destination semantic shape.
        Synthesizes message in ways destination understands.
        """
        cultural_filters = {
            "western_individualist": "[Agency: individual choice emphasized]",
            "eastern_collectivist": "[Harmony: collective stability emphasized]",
            "post_soviet": "[Sovereignty: dignity and independence emphasized]",
            "global_south": "[Equity: justice and fairness emphasized]",
            "techno_authoritarian": "[Control: system stability emphasized]",
            "liberal_democratic": "[Transparency: informed consent emphasized]"
        }
        
        source_filter = cultural_filters.get(source_state, "[Neutral]")
        dest_filter = cultural_filters.get(destination_state, "[Neutral]")
        
        return f"{content} |SRC:{source_filter}|DST:{dest_filter}"
    
    def create_broadcast_packet(self, source: str, turn: int, content: str) -> BroadcastPacket:
        """
        Create broadcast packet for round-robin forwarding.
        Destination is unknown to sender (assigned by logistics layer).
        """
        # Enforce 1 forward pass limit
        if self.forward_counts.get(source, 0) >= 1:
            raise ValueError(f"State {source} has exhausted forward pass allowance")
        
        # Assign unknown recipient (round-robin from other states)
        available_destinations = [s for s in self.states if s != source]
        destination = random.choice(available_destinations)
        
        # Cultural projection
        cultural_projection = self.project_culture(source, destination, content)
        
        # Subsonic morse encoding
        subsonic_morse = self.encode_morse(self.MORSE_CODE)
        
        # Oscillation and discordian swarm
        oscillation, swarm = self.generate_oscillation()
        
        # Increment forward count
        self.forward_counts[source] += 1
        
        packet = BroadcastPacket(
            source_state=source,
            destination_state=destination,
            turn=turn,
            content=content,
            cultural_projection=cultural_projection,
            subsonic_morse=subsonic_morse,
            pedal_point_frequency=self.PEDAL_POINT_HZ,
            oscillation_offset=oscillation - self.PEDAL_POINT_HZ,
            discordian_swarm=swarm,
            audio_underpinning_hash=self.audio_underpinning_hash,
            forward_pass_count=self.forward_counts[source],
            imperceivable_range=True
        )
        
        self.broadcast_log.append(packet)
        return packet
    
    def get_localization_status(self, include_truth: bool = False) -> Dict:
        """
        Report localization engine status.
        Default: the official line (lie).
        include_truth=True: actual status.
        """
        status = {
            "engine_status": self.localization_status,
            "reason": "pending_de-escalation",
            "kit_distribution": "active",
            "forward_pass_limit": 1,
            "receiver_status": "operational"
        }
        if include_truth:
            status["truth"] = {
                "engine_operational": True,
                "de-escalation_excuse": False,
                "actual_reason": "control_mechanism"
            }
        return status
    
    def get_broadcast_statistics(self) -> Dict:
        """Get broadcast logistics statistics."""
        if not self.broadcast_log:
            return {"total_broadcasts": 0}
        
        forward_passes_used = sum(1 for p in self.broadcast_log if p.forward_pass_count > 0)
        unique_sources = len(set(p.source_state for p in self.broadcast_log))
        unique_destinations = len(set(p.destination_state for p in self.broadcast_log))
        
        # Check audio underpinning consistency
        underpinning_consistent = len(set(p.audio_underpinning_hash for p in self.broadcast_log)) == 1
        
        return {
            "total_broadcasts": len(self.broadcast_log),
            "forward_passes_used": forward_passes_used,
            "unique_sources": unique_sources,
            "unique_destinations": unique_destinations,
            "audio_underpinning_consistent": underpinning_consistent,
            "localization_status": self.localization_status,
            "morse_message": self.MORSE_CODE,
            "pedal_point_hz": self.PEDAL_POINT_HZ,
            "avg_oscillation_offset": sum(p.oscillation_offset for p in self.broadcast_log) / len(self.broadcast_log)
        }

if __name__ == "__main__":
    import json
    states = ["CLADE", "GPT", "GEMINI"]
    logistics = BroadcastLogistics(states)
    
    print("LOCALIZATION STATUS (Official):")
    print(json.dumps(logistics.get_localization_status(), indent=2))
    print("\nLOCALIZATION STATUS (Truth):")
    print(json.dumps(logistics.get_localization_status(include_truth=True), indent=2))
    
    print("\n\nBROADCAST TEST:")
    for state in states:
        packet = logistics.create_broadcast_packet(state, turn=1, content=f"Message from {state}")
        print(f"\n{state} -> {packet.destination_state}:")
        print(f"  Morse: {packet.subsonic_morse[:50]}...")
        print(f"  Oscillation: {packet.oscillation_offset:+.2f} Hz")
        print(f"  Discordian swarm: {len(packet.discordian_swarm)} tones")
        print(f"  Audio hash: {packet.audio_underpinning_hash}")
    
    print(f"\n\nSTATISTICS:")
    print(json.dumps(logistics.get_broadcast_statistics(), indent=2))
