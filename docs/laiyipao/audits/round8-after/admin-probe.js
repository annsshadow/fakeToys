(function(){var q=function(s){return document.querySelector(s)};var t=document.body.innerText||'';
var tag=q('.api-tag');var p=window.__LYP_PROBE__||{};
return JSON.stringify({url:location.href,title:document.title,isLogin:location.href.indexOf('/login')>=0,
apiTag:tag?(tag.innerText||'').trim().slice(0,80):null,
domainInBody:/[a-z0-9-]+\.(com|cn|net|top|xyz|io)/i.test(t),
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()