(async function(){
  var p=window.__LYP_PROBE__||{};
  var legend=document.querySelector('[aria-label*=图例]');
  if(legend)legend.click();
  await new Promise(function(r){setTimeout(r,400)});
  var t=document.body.innerText||'';
  return JSON.stringify({legendFound:!!legend,pct25:(t.match(/25%/g)||[]).length,radius120:(t.match(/120/g)||[]).length,
    probe:{pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})
})()