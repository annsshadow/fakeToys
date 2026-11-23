(function(){
 var pr=window.__LYP_PROBE__||{};var t=(document.body.innerText||'');
 var d=[].slice.call(document.querySelectorAll('.defeat-review')).map(function(e){return(e.innerText||'').trim()}).filter(Boolean);
 return JSON.stringify({url:location.href,title:document.title,isLogin:/login/.test(location.hash),textLen:t.length,
  hasDefeatReview:d.length>0,reviewText:d[0]?d[0].slice(0,400):'',
  hasLostWave:/失守/.test(t),hasLeakRatio:/漏怪[ 11]/.test(t)||/漏怪/.test(t),hasNoReaction:/反应[ 0次]+0|反应 0/.test(t),
  hasAction:/补|升|不要|优先/.test(t),hasBasis:/依据/.test(t),hasGrowth:/养成前后/.test(t),
  pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length});
})();