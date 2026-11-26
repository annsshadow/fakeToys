(function(){
  var innerW=window.innerWidth, docW=document.documentElement.scrollWidth;
  var els=Array.prototype.slice.call(document.querySelectorAll('[role="button"]'));
  var vis=function(e){var r=e.getBoundingClientRect();var s=getComputedStyle(e);return r.width>0&&r.height>0&&s.visibility!=="hidden"&&s.display!=="none";};
  var visEls=els.filter(vis);
  return {hash:location.hash,href:location.href,innerW:innerW,docW:docW,hOverflow:docW-innerW,
    rbCount:els.length,visCount:visEls.length,
    visAllTabindex:visEls.every(function(e){return e.hasAttribute('tabindex')&&e.tabIndex>=0;}),
    visAllCursorPointer:visEls.every(function(e){return getComputedStyle(e).cursor==='pointer';}),
    outlines:visEls.slice(0,3).map(function(e){var s=getComputedStyle(e);return s.outlineStyle+' '+s.outlineWidth+' '+s.outlineColor;}),
    cursors:visEls.slice(0,3).map(function(e){return getComputedStyle(e).cursor;}),
    bodyLen:document.body.innerText.length};
})()