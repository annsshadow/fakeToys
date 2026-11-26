(function(){
  var pr=window.__LYP_PROBE__||{};var text=(document.body.innerText||'');var doc=document.documentElement;
  function arr(sel){return [].slice.call(document.querySelectorAll(sel)).map(function(e){return (e.innerText||'').trim()}).filter(Boolean).slice(0,6)}
  return JSON.stringify({
    url:location.href,title:document.title,vw:window.innerWidth,vh:window.innerHeight,textLen:text.length,
    hOverflow:doc.scrollWidth-doc.clientWidth,isLogin:/login/.test(location.hash),
    r12Text:arr('.r12-priority-text'),r12Cta:arr('.r12-priority-cta'),r12Title:arr('.r12-priority-title'),
    hasEquipFit:text.indexOf('装备契合 0/18')>=0||text.indexOf('装备契合0/18')>=0,
    hasWeight21:text.indexOf('权重 21%')>=0,hasGain378:text.indexOf('378')>=0,hasCtaEquip:text.indexOf('去配装备')>=0,
    longTail:(text.match(/[0-9]+[.][0-9]{6,}/g)||[]).slice(0,3),
    pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length
  });
})();