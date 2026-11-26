(function(){
window.__CAP=[];
var obs=new MutationObserver(function(){
 var n=document.querySelectorAll("[role=status],[role=alert],[aria-live]");
 for(var i=0;i<n.length;i++){var e=n[i];var t=String(e.innerText||e.textContent||"").trim();var rec={role:e.getAttribute("role")||"",live:e.getAttribute("aria-live")||"",cls:String(e.className||"").slice(0,50),text:t.slice(0,80),ts:Date.now()};
  if(t&&!window.__CAP.some(function(x){return x.text===rec.text&&x.role===rec.role;}))window.__CAP.push(rec);}
});
obs.observe(document.body,{subtree:true,childList:true,characterData:true,attributes:true});
window.__OBS=obs;
var inp=document.querySelector("input");if(inp){inp.value="QA预设MO";inp.dispatchEvent(new Event("input",{bubbles:true}));}
var link=document.querySelector(".preset-save-row uni-text.link");
if(link){link.click();return "TRIGGERED";}
return "NO_LINK";
})()