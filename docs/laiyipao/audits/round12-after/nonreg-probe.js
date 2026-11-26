(function(){
 var pr=window.__LYP_PROBE__||{};var t=(document.body.innerText||'');
 function q(s){return document.querySelector(s)?1:0}
 return JSON.stringify({url:location.href,title:document.title,
  firstRunGuide:q('.first-run-guide'),guideText:(document.querySelector('.first-run-guide')?.innerText||'').trim().slice(0,60),
  hasAoE25:/25%/.test(t),hasRadius120:/120/.test(t),
  honestAttack:/占比受限|权重上限|元素侧.*主力/.test(t),
  longTail:(t.match(/[0-9]+[.][0-9]{6,}/g)||[]).slice(0,3),
  pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length});
})();