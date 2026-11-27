(function(){
  var res={url:location.href,title:document.title};
  res.isLogin=/login/.test(location.hash||'');
  var els=[].slice.call(document.querySelectorAll('.btn, [role="button"], .r7-btn, .start-btn, .tap'));
  res.items=els.slice(0,16).map(function(b){
    var cs=getComputedStyle(b);
    return {tag:b.tagName,cls:(b.className||'').toString().slice(0,46),role:b.getAttribute('role'),tabIndex:b.tabIndex,pe:cs.pointerEvents,cursor:cs.cursor,txt:(b.textContent||'').trim().slice(0,22)};
  });
  return JSON.stringify(res);
})()
