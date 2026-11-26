(function(){var t=document.body.innerText||'';var p=window.__LYP_PROBE__||{};
return JSON.stringify({hasFalseClaim:t.indexOf('攻击力无关')>=0||t.indexOf('攻击力不影响')>=0,
hasHonest:t.indexOf('元素侧是主力')>=0||t.indexOf('面板攻击力也有贡献')>=0||t.indexOf('权重上限30%')>=0,
excerpt:(function(){var i=t.indexOf('元素侧');return i>=0?t.slice(i,i+120):t.slice(0,120)})(),
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()