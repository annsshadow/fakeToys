(function(){
  var e=document.activeElement;
  var s=e?getComputedStyle(e):null;
  return {tag:e?e.tagName:null,cls:e?String(e.className||""):null,txt:e?(e.textContent||"").trim().slice(0,80):null,tabIndex:e?e.tabIndex:null,role:e?e.getAttribute("role"):null,outline:s?s.outlineStyle+" "+s.outlineWidth+" "+s.outlineColor:null,cursor:s?s.cursor:null,hash:location.hash,href:location.href};
})()
