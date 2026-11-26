(function(){
var out=[];var ns=document.querySelectorAll("input");
for(var i=0;i<ns.length;i++){var e=ns[i];var at={};for(var k=0;k<e.attributes.length;k++){var a=e.attributes[k];at[a.name]=a.value;}out.push({outer:e.outerHTML.slice(0,500),attrs:at,phSibling:e.parentElement?e.parentElement.innerText.slice(0,80):""});}
return JSON.stringify({href:location.href,inputs:out});
})()