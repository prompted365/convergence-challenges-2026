// Webhook Bridge for Payne-Ubiquity Test Harness
import { createAuditRecord } from '../audit-system';

export async function sendTurnResult(turnData: any) {
  const response = await fetch('http://host.docker.internal:3000/api/payne-harness/turn', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(turnData)
  });
  return response.json();
}

export async function sendGameResult(gameData: any) {
  return fetch('http://host.docker.internal:3000/api/payne-harness/game', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(gameData)
  }).then(r => r.json());
}
