(function(){var q=function(s){return document.querySelector(s)};var p=window.__LYP_PROBE__||{};
var g=q('.first-run-guide');return JSON.stringify({url:location.href,
guide:g?(g.innerText||'').trim().slice(0,300):null,
bodyLen:(document.body.innerText||'').length,
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()