(function(){
  var de=document.documentElement;
  var res={url:location.href,title:document.title,isLogin:/login/.test(location.hash||''),innerW:window.innerWidth,clientW:de.clientWidth,scrollW:de.scrollWidth};
  res.hOverflow=de.scrollWidth>de.clientWidth+1;
  var links=[].slice.call(document.querySelectorAll('.link,[class*="link"]'));
  res.linkCount=links.length;
  res.links=links.map(function(el,i){var cs=getComputedStyle(el);return {i:i,tag:el.tagName,txt:(el.textContent||'').trim().slice(0,26),role:el.getAttribute('role'),tabAttr:el.getAttribute('tabindex'),tabIndex:el.tabIndex,cursor:cs.cursor};});
  return JSON.stringify(res);
})()
