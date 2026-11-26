(function(){
  var ae=document.activeElement;
  var q=function(s){var e=document.querySelector(s);if(!e)return null;var r=e.getBoundingClientRect();var st=getComputedStyle(e);return {exists:true,txt:(e.innerText||e.textContent||"").trim().slice(0,80),visible:r.width>0&&r.height>0&&st.visibility!=="hidden"&&st.display!=="none",display:st.display,h:Math.round(r.height)};};
  return {hash:location.hash,href:location.href,scrollY:Math.round(window.scrollY||document.documentElement.scrollTop||0),bodyLen:document.body.innerText.length,active:{tag:ae?ae.tagName:null,cls:ae?String(ae.className||""):null,txt:ae?(ae.innerText||ae.textContent||"").trim().slice(0,60):null,role:ae?ae.getAttribute("role"):null},whyDetail:q(".r6-why-detail"),whyBody:q(".r6-why-body"),applyDone:q(".apply-done"),compare:q(".r7-compare,.r7-modal,.r7-panel"),toast:q(".toast,.uni-toast"),title:q(".r6-why-toggle"),count:document.querySelectorAll("[role=button]").length};
})()
