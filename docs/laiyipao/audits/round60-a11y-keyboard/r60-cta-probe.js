(function(){
  var all=Array.prototype.slice.call(document.querySelectorAll("text,view,uni-text,uni-view"));
  var pick=function(kw){var e=all.find(function(x){var t=(x.innerText||x.textContent||"");return t.indexOf(kw)>=0&&x.children.length===0;});if(!e)return null;var r=e.getBoundingClientRect();var st=getComputedStyle(e);return {txt:(e.innerText||e.textContent||"").trim().slice(0,70),cls:String(e.className||""),role:e.getAttribute("role"),tab:e.tabIndex,hasTab:e.hasAttribute("tabindex"),cursor:st.cursor,visible:r.width>0&&r.height>0,attrSelf:e.getAttribute("data-r14-source")};};
  return {hash:location.hash,href:location.href,innerW:innerWidth,docW:document.documentElement.scrollWidth,selfCta:pick("在本页强化"),r12Cta:pick("去配装备"),r20Cta:pick("去战斗"),aiCta:pick("去看变更影响"),detailCta:pick("看战报详情")};
})()
