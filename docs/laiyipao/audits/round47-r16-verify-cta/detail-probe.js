(function() { try {
  const cta = document.querySelector('.r16-timeline-cta');
  const drawer = document.querySelector('.el-drawer');
  const rows = [].slice.call(document.querySelectorAll('.verify-row'));
  const rowRects = rows.map(function(r) { var q = r.getBoundingClientRect(); return { top: Math.round(q.top), bottom: Math.round(q.bottom), text: (r.innerText||'').replace(/\s+/g,' ').trim().slice(0,160) }; });
  const hashEls = [].slice.call(document.querySelectorAll('code.hash'));
  const timeline = (function(){ var t = cta ? cta.parentElement : null; while (t && !/AI · 反应时间线/.test(t.innerText||'')) t = t.parentElement; return t ? (t.innerText||'').replace(/\s+/g,' ').trim().slice(0,1000) : ''; })();
  const verifyInputs = [].slice.call(document.querySelectorAll('.el-drawer input')).map(function(i){ return { placeholder: i.placeholder||'', value: i.value||'', type: i.type||'' }; });
  return JSON.stringify({
    href: location.href, title: document.title, innerW: innerWidth, docW: document.documentElement.scrollWidth,
    isLogin: location.href.indexOf('/admin/login') >= 0,
    drawerOpen: !!drawer && !!drawer.offsetParent,
    ctaText: cta ? (cta.innerText||'').trim() : '',
    ctaRect: cta ? (function(){ var r=cta.getBoundingClientRect(); return { x:r.x, y:r.y, w:r.width, h:r.height, cx:r.x+r.width/2, cy:r.y+r.height/2 }; })() : null,
    ctaCursor: cta ? getComputedStyle(cta).cursor : '',
    ctaClickAttr: cta ? !!cta.getAttribute('onclick') : false,
    verifyRows: rowRects,
    verifyRowInViewport: rowRects.some(function(r){ return r.top >= 0 && r.bottom <= innerHeight; }),
    hashTexts: hashEls.map(function(e){ return (e.innerText||'').trim(); }),
    verifyInputs: verifyInputs,
    timelineText: timeline,
    scrollY: Math.round(scrollY||0), bodyLen: (document.body.innerText||'').length,
    resultText: (function(){ var e=document.querySelector('.verify-result'); return e ? (e.innerText||'').trim().slice(0,400) : ''; })()
  });
 } catch(e) { return JSON.stringify({error:String(e)}); } })()