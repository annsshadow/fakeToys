(function(){
  var doc=document.documentElement;
  var text=(document.body.innerText||'').trim();
  var longTail=(text.match(/[0-9]+\.[0-9]{6,}/g)||[]).slice(0,5);
  var hudPills=[].slice.call(document.querySelectorAll('.hud-pill,.star-gap-text')).map(function(e){var r=e.getBoundingClientRect();return {t:(e.innerText||'').trim().slice(0,24),w:Math.round(r.width),h:Math.round(r.height)}}).slice(0,6);
  var growth=document.querySelector('.r10-growth,.r10-gem-gain');
  var growthText=growth?(growth.innerText||'').trim().slice(0,220):'';
  var nums=[].slice.call(document.querySelectorAll('.r7-bar-val,.r7-delta,.r7-star,.r7-vs-label,.r10-growth-num,.r10-gem-key,.r10-gem-delta')).map(function(e){return (e.innerText||'').trim()}).filter(Boolean).slice(0,14);
  var pr=window.__LYP_PROBE__||{};
  return JSON.stringify({url:location.href,title:document.title,vw:window.innerWidth,vh:window.innerHeight,textLen:text.length,
    hOverflow:doc.scrollWidth-doc.clientWidth,longTail:longTail,longTailFlag:longTail.length>0,
    hudPills:hudPills,growthCard:!!growth,growthText:growthText,nums:nums,
    isLogin:/login/.test(location.hash),
    pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length,
    peMsgs:(pr.pageErrors||[]).slice(0,2)});
})();
