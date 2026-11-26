(() => {
  const p=window.__LYP_PROBE__||{};
  const brief=document.querySelector('.r26-wave-brief');
  const cards=[...document.querySelectorAll('.hand-card')];
  const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  const r=document.querySelector('.result-title');
  const s=r?'SETTLED':((brief&&cards.length)?'CARD_PHASE':((brief&&skip)?'SKIP_ONLY':'WAIT'));
  return s+JSON.stringify({state:s,pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,settle:(p.settleResponses||[]).slice(-2),hashResponses:(p.hashResponses||[]).slice(-3)});
})()