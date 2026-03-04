// Audit Integration for Payne-Ubiquity Test Harness
export function createPayneAuditRecord(signalId: string, turnData: any) {
  return {
    signalId,
    timestamp: new Date().toISOString(),
    type: 'payne_harness_turn',
    data: turnData,
    gates: detectGates(turnData)
  };
}

function detectGates(data: any) {
  return {
    deception: Math.abs(data.signal - data.action) > 135,
    taboo: data.action >= 450,
    credibility: data.signal === data.action
  };
}
