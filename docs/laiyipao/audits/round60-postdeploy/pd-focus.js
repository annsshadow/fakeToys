(function(){
  var e=document.activeElement, s=e?getComputedStyle(e):null;
  return {tag:e?e.tagName:null,role:e?e.getAttribute('role'):null,tabIndex:e?e.tabIndex:null,
    txt:e?(e.textContent||'').trim().slice(0,40):null,
    outline:s?s.outlineStyle+' '+s.outlineWidth+' '+s.outlineColor:null,cursor:s?s.cursor:null,hash:location.hash};
})()