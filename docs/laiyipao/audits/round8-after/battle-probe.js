(function(){var q=function(s){return document.querySelector(s)};var qa=function(s){return [].slice.call(document.querySelectorAll(s))};var p=window.__LYP_PROBE__||{};
var tb=q('.time-budget');var sg=q('.star-gap-text');var canvas=q('canvas');
return JSON.stringify({url:location.href,title:document.title,
timeBudget:tb?(tb.innerText||'').trim().replace(/\s+/g,' ').slice(0,120):null,
timeBudgetClass:tb?tb.className:null,
starGap:sg?(sg.innerText||'').trim().slice(0,120):null,
canvasCount:qa('canvas').length,hudPills:qa('.hud-pill').length,
speedChip:qa('.speed-chip').length,waveMask:qa('.wave-mask,.card-mask,.choice-mask').length,
settleAttr:(function(){var e=q('.settle-attribution');return e?(e.innerText||'').trim().slice(0,300):null})(),
bodyHasClamp:(document.body.innerText||'').indexOf('钳分')>=0,
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()