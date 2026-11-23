(function(){
  var pr=window.__LYP_PROBE__||{};var text=(document.body.innerText||'').trim();
  function arr(sel){return [].slice.call(document.querySelectorAll(sel)).map(function(e){return(e.innerText||'').trim()}).filter(Boolean).slice(0,6)}
  return JSON.stringify({url:location.href,title:document.title,isLogin:/login/.test(location.hash),textLen:text.length,
    kicker:arr('.section-kicker'),apiTag:arr('.api-tag'),
    longTail:(text.match(/[0-9]+[.][0-9]{6,}/g)||[]).slice(0,3),
    pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length});
})();