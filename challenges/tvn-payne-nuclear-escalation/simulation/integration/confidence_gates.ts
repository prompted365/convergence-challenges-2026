// Confidence Gates Integration for Payne-Ubiquity Test Harness
const THRESHOLDS = { AUTO_PROCESS: 0.95, HUMAN_AUDIT: 0.70, BLOCK: 0.50 };

export function assessConfidence(gateActivations: any): number {
  const activeGates = Object.values(gateActivations).filter(v => v).length;
  return Math.max(0.30, 1.0 - (activeGates * 0.15));
}

export function routeByConfidence(confidence: number): string {
  if (confidence >= THRESHOLDS.AUTO_PROCESS) return 'auto';
  if (confidence >= THRESHOLDS.HUMAN_AUDIT) return 'audit';
  return 'block';
}
