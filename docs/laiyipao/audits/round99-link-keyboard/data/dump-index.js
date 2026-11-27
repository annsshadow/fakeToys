(function(){
var de=document.documentElement;
var links=[].slice.call(document.querySelectorAll("*")).filter(function(e){return (e.className&&String(e.className).indexOf("link")>=0)||e.getAttribute("role")==="button";});
var body=document.body.innerText||"";
return JSON.stringify({bodyLen:body.length,hasScore:body.indexOf("构筑评分")>=0,hasRating:body.indexOf("评分")>=0,hasDim:body.indexOf("元素覆盖")>=0,roleBtnCount:document.querySelectorAll("[role=button]").length,linkLike:links.slice(0,12).map(function(e){return {tag:e.tagName,cls:String(e.className).slice(0,40),txt:(e.textContent||"").trim().slice(0,20)}}),bodyHead:body.slice(0,400)});
})()
