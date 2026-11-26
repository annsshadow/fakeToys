(function(){var q=function(s){return document.querySelector(s)};var qa=function(s){return [].slice.call(document.querySelectorAll(s))};var p=window.__LYP_PROBE__||{};var u=location.href;var t=document.body.innerText||'';
return JSON.stringify({url:u,title:document.title,isLogin:u.indexOf('/login')>=0,bodyLen:t.length,
apiTag:q('.api-tag')?(q('.api-tag').innerText||'').trim():null,domainInBody:/[a-z0-9-]+\.(com|cn|net|top|xyz|io)/i.test(t),
kicker:q('.section-kicker')?(q('.section-kicker').innerText||'').trim().slice(0,60):null,
aiCard:qa('.ai-insight-card,.r6-rec,.ai-card').length,cmdTrigger:qa('.command-trigger').length,
wiki:u.indexOf('/wiki')>=0?1:0,aiBadge:(t.match(/AI/g)||[]).length,
pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});})()