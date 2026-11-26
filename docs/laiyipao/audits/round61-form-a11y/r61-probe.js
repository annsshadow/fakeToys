(function(){
function vis(el){var cs=getComputedStyle(el);if(cs.display==="none"||cs.visibility==="hidden"||parseFloat(cs.opacity)===0)return false;var r=el.getBoundingClientRect();return r.width>=1&&r.height>=1;}
function txt(el){return String(el.innerText||el.textContent||"").trim();}
function accName(el){
 var lb=el.getAttribute("aria-labelledby");if(lb){var ids=lb.split(" ");var p=[];for(var i=0;i<ids.length;i++){var n=document.getElementById(ids[i]);if(n)p.push(txt(n));}if(p.length)return {src:"labelledby",text:p.join(" ")};}
 var al=el.getAttribute("aria-label");if(al)return {src:"aria-label",text:al};
 var id=el.getAttribute("id");if(id){var ls=document.querySelectorAll("label");for(var j=0;j<ls.length;j++){if(ls[j].getAttribute("for")===id)return {src:"label[for]",text:txt(ls[j])};}}
 var w=el.closest?el.closest("label"):null;if(w)return {src:"wrapping-label",text:txt(w)};
 var ph=el.getAttribute("placeholder");if(ph)return {src:"placeholder",text:ph};
 return {src:"NONE",text:""};
}
var ins=[];var il=document.querySelectorAll("input,textarea,select");
for(var i=0;i<il.length;i++){var el=il[i];if(!vis(el))continue;var r=el.getBoundingClientRect();ins.push({tag:el.tagName,type:el.getAttribute("type")||"",cls:String(el.className||"").slice(0,60),val:String(el.value||"").slice(0,30),acc:accName(el),phClass:el.getAttribute("placeholder-class")||"",tab:el.tabIndex,x:Math.round(r.x),y:Math.round(r.y),w:Math.round(r.width),h:Math.round(r.height)});}
var fb=[];var fl=document.querySelectorAll("[role=alert],[role=status],[aria-live]");
for(var i=0;i<fl.length;i++){var el=fl[i];if(!vis(el))continue;fb.push({tag:el.tagName,role:el.getAttribute("role")||"",live:el.getAttribute("aria-live")||"",cls:String(el.className||"").slice(0,50),text:txt(el).slice(0,80)});}
var rb=[];var rl=document.querySelectorAll("[role=button]");
for(var i=0;i<rl.length;i++){var el=rl[i];if(!vis(el))continue;rb.push({cls:String(el.className||"").slice(0,45),tab:el.getAttribute("tabindex"),kd:!!el.getAttribute("@keydown.enter")||!!el.getAttribute("data-kd"),text:txt(el).slice(0,30)});}
var over=[];var all=document.querySelectorAll("body *");
for(var i=0;i<all.length;i++){var el=all[i];if(!vis(el))continue;var cs=getComputedStyle(el);if(cs.position==="absolute"||cs.position==="fixed")continue;var r=el.getBoundingClientRect();if(r.right>window.innerWidth+2||r.left<-2){var anc=el.parentElement;var clipped=false;while(anc){var acs=getComputedStyle(anc);if(acs.overflowX==="hidden"||acs.overflowX==="auto"||acs.overflowX==="scroll"){clipped=true;break;}anc=anc.parentElement;}if(!clipped)over.push({tag:el.tagName,cls:String(el.className||"").slice(0,40),left:Math.round(r.left),right:Math.round(r.right)});}}
return JSON.stringify({href:location.href,hash:location.hash,innerW:window.innerWidth,docW:document.documentElement.scrollWidth,bodyW:document.body.scrollWidth,inputCount:ins.length,noName:ins.filter(function(x){return x.acc.src==="NONE"||!x.acc.text;}).length,inputs:ins,feedbackCount:fb.length,feedback:fb.slice(0,40),roleButtonCount:rb.length,roleButtonNoTab:rb.filter(function(x){return x.tab===null||x.tab!==0;}).length,overflowCount:over.length,overflow:over.slice(0,10),bodyLen:document.body.innerText.length});
})()