(function(){
function vis(el){var cs=getComputedStyle(el);if(cs.display==='none'||cs.visibility==='hidden'||parseFloat(cs.opacity)===0)return false;var r=el.getBoundingClientRect();return r.width>=1&&r.height>=1;}
function txt(el){return String(el.innerText||el.textContent||'').trim();}
var ins=[];var il=document.querySelectorAll('input,textarea,select');
for(var i=0;i<il.length;i++){var el=il[i];if(!vis(el))continue;var r=el.getBoundingClientRect();ins.push({tag:el.tagName,type:el.getAttribute('type')||'',nativeAria:el.getAttribute('aria-label')||'',ph:el.getAttribute('placeholder')||'',tab:el.tabIndex,w:Math.round(r.width),h:Math.round(r.height)});}
var fb=[];var fl=document.querySelectorAll('[role=alert],[role=status],[aria-live]');
for(var i=0;i<fl.length;i++){var el=fl[i];if(!vis(el))continue;fb.push({role:el.getAttribute('role')||'',live:el.getAttribute('aria-live')||'',text:txt(el).slice(0,80)});}
return JSON.stringify({hash:location.hash,innerW:window.innerWidth,docW:document.documentElement.scrollWidth,hOverflow:document.documentElement.scrollWidth-window.innerWidth,inputCount:ins.length,inputs:ins,nativeAriaAll:ins.every(function(x){return !!x.nativeAria;}),feedbackCount:fb.length,feedback:fb,bodyLen:document.body.innerText.length});
})()
