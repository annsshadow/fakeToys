(function(){var q=function(s){return document.querySelector(s)};var qa=function(s){return [].slice.call(document.querySelectorAll(s))};
var p=window.__LYP_PROBE__||{};var t=document.body.innerText||'';
return JSON.stringify({url:location.href,title:document.title,
impact:qa('.r7-impact').length,impactTitle:(function(){var e=q('.r7-impact-title');return e?(e.innerText||'').trim().slice(0,100):null})(),
buttons:qa('.r7-btn').map(function(e){return(e.innerText||'').trim()}).slice(0,8),
honestEmpty:t.indexOf('本地规则引擎没有真实推荐')>=0||t.indexOf('收益数据不可用')>=0,
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()