(function(){
  var href=location.href, hash=location.hash;
  var innerW=window.innerWidth, docW=document.documentElement.scrollWidth;
  var els=Array.prototype.slice.call(document.querySelectorAll('[role="button"]'));
  var vis=function(e){var r=e.getBoundingClientRect();var s=getComputedStyle(e);return r.width>0&&r.height>0&&s.visibility!=="hidden"&&s.display!=="none";};
  var desc=function(e){var r=e.getBoundingClientRect();var s=getComputedStyle(e);return {tag:e.tagName,cls:String(e.className||""),txt:(e.textContent||"").trim().slice(0,60),tabIndex:e.tabIndex,hasAttr:e.hasAttribute("tabindex"),attr:e.getAttribute("tabindex"),cursor:s.cursor,outline:s.outlineStyle+" "+s.outlineWidth,rect:{x:Math.round(r.x),y:Math.round(r.y),w:Math.round(r.width),h:Math.round(r.height)},role:e.getAttribute("role")};};
  var visEls=els.filter(vis).map(desc);
  var ae=document.activeElement;
  return {href:href,hash:hash,innerW:innerW,docW:docW,hOverflow:docW-innerW,rbCount:els.length,visCount:visEls.length,visAllTabindex:visEls.every(function(e){return e.hasAttr&&e.tabIndex>=0;}),visAllCursorPointer:visEls.every(function(e){return e.cursor==="pointer";}),visEls:visEls.slice(0,15),active:{tag:ae?ae.tagName:null,cls:ae?String(ae.className||""):null,tabIndex:ae?ae.tabIndex:null,role:ae?ae.getAttribute("role"):null,txt:ae?(ae.textContent||"").trim().slice(0,60):null},bodyLen:document.body.innerText.length};
})()
