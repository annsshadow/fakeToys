(function(){
  var p=window.__LYP_PROBE__||{};
  var app=document.querySelector('#app')||document.body;
  return JSON.stringify({
    url:location.href,textLen:(document.body.innerText||'').length,
    appHtmlLen:(app.innerHTML||'').length,
    appChildren:app.children.length,
    pe:(p.pageErrors||[]).slice(-3),ce:(p.consoleErrors||[]).slice(-3),
    af:(p.apiFailures||[]).slice(-3),fr:(p.failedRequests||[]).slice(-3)
  })
})()