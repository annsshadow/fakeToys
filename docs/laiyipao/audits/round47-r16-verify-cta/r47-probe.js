(function() {
  try {
    var cta = document.querySelector('.r16-timeline-cta');
    var drawer = document.querySelector('.el-drawer');
    var body = document.querySelector('.el-drawer__body');
    var rows = [].slice.call(document.querySelectorAll('.verify-row'));
    var rowRects = rows.map(function(r){ var q=r.getBoundingClientRect(); return {top:Math.round(q.top),bottom:Math.round(q.bottom),text:(r.innerText||'').replace(/\s+/g,' ').trim().slice(0,160)}; });
    var hashEls = [].slice.call(document.querySelectorAll('.el-drawer code.hash'));
    var inputs = [].slice.call(document.querySelectorAll('.el-drawer input')).map(function(i){return {placeholder:i.placeholder||'',value:i.value||''};});
    var tl = document.querySelector('.r16-reaction-timeline');
    return JSON.stringify({
      href: location.href,
      innerW: innerWidth, innerH: innerHeight,
      isLogin: location.href.indexOf('/admin/login')>=0,
      drawerOpen: !!drawer && !!drawer.offsetParent,
      drawerScrollTop: body ? Math.round(body.scrollTop) : null,
      drawerScrollH: body ? Math.round(body.scrollHeight) : null,
      drawerClientH: body ? Math.round(body.clientHeight) : null,
      windowScrollY: Math.round(window.scrollY||0),
      ctaText: cta ? (cta.innerText||'').trim() : '',
      ctaRect: cta ? (function(){var r=cta.getBoundingClientRect();return {x:Math.round(r.x),y:Math.round(r.y),w:Math.round(r.width),h:Math.round(r.height),cx:Math.round(r.x+r.width/2),cy:Math.round(r.y+r.height/2)};})() : null,
      ctaCursor: cta ? getComputedStyle(cta).cursor : '',
      verifyRows: rowRects,
      verifyRowInViewport: rowRects.some(function(r){return r.top>=0 && r.bottom<=innerHeight;}),
      hashes: hashEls.map(function(e){return (e.innerText||'').trim();}),
      inputs: inputs,
      timelineText: tl ? (tl.innerText||'').replace(/\s+/g,' ').trim().slice(0,600) : '',
      verifyResultCount: document.querySelectorAll('.verify-row .el-tag').length,
      bodyLen: (document.body.innerText||'').length
    });
  } catch(e) { return JSON.stringify({error:String(e)}); }
})()
