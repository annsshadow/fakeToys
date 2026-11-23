(function(){
  var pr=window.__LYP_PROBE__||{};var text=(document.body.innerText||'').trim();
  return JSON.stringify({url:location.href,title:document.title,
    isLogin:/login/.test(location.hash),textLen:text.length,
    hasAI:text.indexOf('AI')>=0,kicker:[].slice.call(document.querySelectorAll('.section-kicker')).map(function(e){return (e.innerText||'').trim()}).slice(0,4),
    apiTag:(function(){var e=document.querySelector('.api-tag');return e?(e.innerText||'').trim():''})(),
    pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length});
})();
