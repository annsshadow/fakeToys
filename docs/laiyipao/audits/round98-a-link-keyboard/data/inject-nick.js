(function(){
 var e=Array.from(document.querySelectorAll(".link")).find(function(x){return (x.textContent||"").trim()==="改名";});
 if(!e) return JSON.stringify({ok:false});
 e.setAttribute("tabindex","0");e.focus();
 return JSON.stringify({ok:true,active:document.activeElement===e,tab:e.getAttribute("tabindex")});
})()
