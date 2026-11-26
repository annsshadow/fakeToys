(function(){
  var t=document.body.innerText||'';
  var honest=/元素侧.*主力|权重上限30%|面板攻击力.*占比受限/.test(t);
  var falseClaim=/攻击力.*无关|和攻击力没关系/.test(t);
  var p=window.__LYP_PROBE__||{};
  return JSON.stringify({url:location.href,honest:honest,falseClaim:falseClaim,
    firstRun:document.querySelectorAll('.first-run-guide').length,
    textLen:t.length,probe:{pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})
})()