(function(){
  var a=document.activeElement;
  var ov=document.querySelector('.legend-overlay');
  var chip=document.querySelector('.help-chip');
  var close=document.querySelector('.legend-close');
  var pauseOv=document.querySelector('.pause-overlay');
  var drawer=document.querySelector('.reaction-timeline-overlay, .timeline-overlay, .reaction-drawer, .r29-timeline-overlay');
  function desc(el){if(!el)return null;var cls=(el.className&&el.className.baseVal!==undefined?el.className.baseVal:el.className)||'';return {tag:el.tagName,cls:String(cls),text:(el.innerText||el.textContent||'').trim().slice(0,50),role:el.getAttribute&&el.getAttribute('role')};}
  var inOv=!!(ov&&a&&ov.contains(a));
  var out={hash:location.hash,active:desc(a),activeInOverlay:inOv,overlayRole:ov?ov.getAttribute('role'):null,ariaModal:ov?ov.getAttribute('aria-modal'):null,ariaLabel:ov?ov.getAttribute('aria-label'):null,overlayOpen:!!ov,chipRole:chip?chip.getAttribute('role'):null,chipTab:chip?chip.getAttribute('tabindex'):null,closeRole:close?close.getAttribute('role'):null,closeTab:close?close.getAttribute('tabindex'):null,paused:!!pauseOv,drawerOpen:!!drawer,feedCta:!!document.querySelector('.r28-feed-cta'),hOverflow:(document.documentElement.scrollWidth<=document.documentElement.clientWidth+1),innerW:window.innerWidth,docW:document.documentElement.scrollWidth,clientW:document.documentElement.clientWidth,errors:window.__qaErrors||[],bodyLen:(document.body.innerText||'').length};
  console.log('PD_PROBE '+JSON.stringify(out));return out;
})()