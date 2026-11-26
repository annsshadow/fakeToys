(function(){
  var b=document.body;
  var t=b.innerText||b.textContent||'';
  function q(s){return document.querySelectorAll(s).length}
  var p=window.__LYP_PROBE__||{};
  return JSON.stringify({
    url:location.href,title:document.title,
    isLogin:/login|登录/.test((location.hash||'')+' '+(document.title||'')),
    hasToken:(localStorage.getItem('lyp_token')||'').length>0,
    textLen:t.length,
    kicker:q('.section-kicker'),kpi:q('.r6-kpi'),ai:q('.r6-ai-card,.ai-insight-card,.r7-impact,.r10-growth'),
    r10growth:q('.r10-growth'),r7impact:q('.r7-impact'),basisText:/依据|来源|近7天/.test(t),
    ctas:[].slice.call(document.querySelectorAll('.r6-cta,.cta,.r7-btn')).map(function(e){return (e.innerText||'').trim()}).filter(Boolean).slice(0,8),
    growthText:(function(){var e=document.querySelector('.r10-growth');return e?(e.innerText||'').slice(0,300):''})(),
    tabs:[].slice.call(document.querySelectorAll('.tab-item,.bag-tab')).map(function(e){return (e.innerText||'').trim()}).filter(Boolean).slice(0,10),
    probe:{pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}
  })
})()