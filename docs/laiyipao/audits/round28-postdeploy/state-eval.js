(() => {
  const p=window.__LYP_PROBE__||{};
  const rr=document.querySelector('.result-title');
  const cards=document.querySelectorAll('.hand-card');
  const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  const r26=document.querySelector('.r26-wave-brief');
  const s=rr?'SETTLED':((cards.length&&r26)?'CARD_PHASE':(skip?'SKIP_ONLY':'WAIT'));
  return s+JSON.stringify({state:s,pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,settle:(p.settleResponses||[]).slice(-2)});
})()
