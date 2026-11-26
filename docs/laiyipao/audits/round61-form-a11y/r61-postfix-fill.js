(function(){
var n=document.querySelectorAll('input,textarea,select');var out=[];
for(var i=0;i<n.length;i++){var e=n[i];var cs=getComputedStyle(e);if(cs.display==='none'||cs.visibility==='hidden')continue;var r=e.getBoundingClientRect();if(r.width<1||r.height<1)continue;
var v='QA填写'+i+'号';e.value=v;e.dispatchEvent(new Event('input',{bubbles:true}));e.dispatchEvent(new Event('change',{bubbles:true}));out.push({i:i,val:e.value,aria:e.getAttribute('aria-label')||''});}
return JSON.stringify({filled:out});
})()