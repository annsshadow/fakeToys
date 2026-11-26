(function(){
  var o={url:location.href,hits:(window.__qaHits||[]).map(function(x){return x.u;}),hitCount:(window.__qaHits||[]).length,kvList:[],recordTime:null,recordRaw:null,hashLine:null,snapshotCount:0,r5Conclusion:null,errText:null,hOverflow:0,innerW:0,docW:0,clientW:0,bodyLen:0,hasT:false,hasZ:false};
  try{
    var k=document.querySelectorAll(".kv");
    for(var i=0;i<k.length;i++){var t=(k[i].textContent||"").replace(/\s+/g," ").trim();o.kvList.push(t);if(t.indexOf("记录时间")>=0)o.recordTime=t;if(t.indexOf("哈希")>=0)o.hashLine=t;}
    if(o.recordTime){o.recordRaw=o.recordTime.replace(/^记录时间/,"");o.hasT=o.recordRaw.indexOf("T")>=0;o.hasZ=o.recordRaw.indexOf("Z")>=0;}
    var e=document.querySelector("[role=alert]");o.errText=e?(e.textContent||"").trim():null;
    o.snapshotCount=document.querySelectorAll(".bs-item").length;
    var r5=document.querySelector(".r5-note-text");o.r5Conclusion=r5?(r5.textContent||"").trim():null;
    o.innerW=window.innerWidth;o.docW=document.documentElement.scrollWidth;o.clientW=document.documentElement.clientWidth;
    o.hOverflow=o.docW-o.clientW;o.bodyLen=document.body.innerText.length;
  }catch(ex){o.error=String(ex);}
  console.log("PD_RESULT "+JSON.stringify(o));return o;
})();
