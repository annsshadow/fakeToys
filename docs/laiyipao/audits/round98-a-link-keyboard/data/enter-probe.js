(function(){
 var els=Array.from(document.querySelectorAll(".link"));
 var e=els.find(function(x){return (x.textContent||"").indexOf("保存当前构筑")>=0;});
 if(!e) return JSON.stringify({ok:false,reason:"notfound",all:els.map(function(x){return (x.textContent||"").trim()})});
 e.setAttribute("tabindex","0"); e.focus();
 var active=document.activeElement===e;
 var before=document.body.textContent.indexOf("请输入预设名");
 e.dispatchEvent(new KeyboardEvent("keydown",{key:"Enter",bubbles:true}));
 return JSON.stringify({ok:true,active:active,tag:e.tagName,tab:e.getAttribute("tabindex"),hash:location.href});
})()
