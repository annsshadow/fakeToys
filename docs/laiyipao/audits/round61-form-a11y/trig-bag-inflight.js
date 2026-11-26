(function(){
var i=document.querySelector("input.preset-input")||document.querySelector("input");
if(i){i.value="QA预设B"+Date.now();i.dispatchEvent(new Event("input",{bubbles:true}));}
var t=document.querySelectorAll(".link");var clicked=false;
for(var k=0;k<t.length;k++){if((t[k].innerText||"").indexOf("保存当前构筑")>=0){t[k].click();clicked=true;break;}}
if(!clicked)return JSON.stringify({err:"NO_LINK"});
var seen=[];var start=Date.now();
function scan(){var n=document.querySelectorAll("[role=status],[role=alert],[aria-live]");var r=[];for(var k=0;k<n.length;k++){var e=n[k];var cs=getComputedStyle(e);if(cs.display==="none")continue;var tx=String(e.innerText||e.textContent||"").trim();if(tx)r.push({role:e.getAttribute("role")||"",live:e.getAttribute("aria-live")||"",cls:String(e.className||"").slice(0,40),text:tx.slice(0,60)});}return r;}
var interval=setInterval(function(){var s=scan();for(var k=0;k<s.length;k++){if(s[k].text.indexOf("保存中")>=0)seen.push({t:Date.now()-start,node:s[k]});}if(Date.now()-start>3000){clearInterval(interval);console.log("INFLIGHT_RESULT="+JSON.stringify({clicked:true,seen:seen.slice(0,3),total:seen.length}));done=true;}},40);
var done=false;return "POLLING_STARTED";
})()