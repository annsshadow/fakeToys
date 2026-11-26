(function(){
var out=[];var ns=document.querySelectorAll("input");
for(var i=0;i<ns.length;i++){var e=ns[i];var chain=[];var p=e;
for(var k=0;k<5&&p;k++){var at={};for(var q=0;q<p.attributes.length;q++){var a=p.attributes[q];if(a.name.indexOf("aria")>=0||a.name==="class"||a.name==="placeholder"||a.name==="role")at[a.name]=a.value;}
chain.push({tag:p.tagName,cls:String(p.className||"").slice(0,60),attrs:at});p=p.parentElement;}
out.push({inputVal:String(e.value||""),chain:chain});}
var allAl=[];var al=document.querySelectorAll("[aria-label]");
for(var i=0;i<al.length;i++){var e=al[i];allAl.push({tag:e.tagName,cls:String(e.className||"").slice(0,50),label:e.getAttribute("aria-label"),isInput:e.tagName==="INPUT"});}
return JSON.stringify({href:location.href,inputChains:out,ariaLabelNodes:allAl,ariaLabelTotal:al.length});
})()