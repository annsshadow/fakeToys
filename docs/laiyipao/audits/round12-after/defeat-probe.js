(function(){
  var pr=window.__LYP_PROBE__||{};var text=(document.body.innerText||'');
  function arr(sel){return [].slice.call(document.querySelectorAll(sel)).map(function(e){return (e.innerText||e.textContent||'').trim()}).filter(Boolean).slice(0,2)}
  var settle=arr('.settle-defeat-review'),report=arr('.defeat-review');var all=settle.concat(report).join(' ');
  return JSON.stringify({
    url:location.href,title:document.title,vw:window.innerWidth,vh:window.innerHeight,isLogin:/login/.test(location.hash),
    hasSettleReview:settle.length>0,hasReportReview:report.length>0,
    hasLostWave:/失守/.test(all),hasLeakRatio:/漏怪/.test(all),hasNoReaction:/反应[ ]*0/.test(all),
    hasCardSkip:/跳过|跳卡|选卡/.test(all),hasAction:/补|升|不要|调整|优先/.test(all),hasBasis:/依据/.test(all),
    settleText:settle[0]?settle[0].slice(0,320):'',reportText:report[0]?report[0].slice(0,320):'',
    pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length
  });
})();