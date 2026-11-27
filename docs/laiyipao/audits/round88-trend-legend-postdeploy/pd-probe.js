(function(){
  var o={url:location.href,hits:window.__qaHits||[],legendText:null,trendExists:false,
    lineCount:0,leakCount:0,winCount:0,loseCount:0,
    lineStroke:null,leakStroke:null,leakDash:null,winFill:null,loseFill:null,
    svgAria:null,innerW:innerWidth,docW:document.documentElement.scrollWidth,
    clientW:document.documentElement.clientWidth,overflowPx:0,hOverflow:false,
    bodyLen:(document.body.textContent||"").length,errText:null};
  try{
    var lg=document.querySelector(".trend-legend");
    o.legendText=lg?(lg.textContent||"").replace(/\s+/g," ").trim():null;
    o.trendExists=!!document.querySelector(".trend-card");
    var svg=document.querySelector(".trend-svg"); o.svgAria=svg?svg.getAttribute("aria-label"):null;
    o.lineCount=document.querySelectorAll(".trend-line").length;
    var tline=document.querySelector(".trend-line"); o.lineStroke=tline?getComputedStyle(tline).stroke:null;
    o.leakCount=document.querySelectorAll(".leak-line").length;
    var lline=document.querySelector(".leak-line"); o.leakStroke=lline?getComputedStyle(lline).stroke:null; o.leakDash=lline?getComputedStyle(lline).strokeDasharray:null;
    var wins=document.querySelectorAll(".trend-win"), loses=document.querySelectorAll(".trend-lose");
    o.winCount=wins.length; o.loseCount=loses.length;
    if(wins.length) o.winFill=getComputedStyle(wins[0]).fill;
    if(loses.length) o.loseFill=getComputedStyle(loses[0]).fill;
    o.overflowPx=o.docW-o.clientW; o.hOverflow=o.overflowPx>1;
    var e=document.querySelector("[role=alert]"); o.errText=e?(e.textContent||"").trim():null;
  }catch(ex){o.error=String(ex);}
  console.log("PROBE_RESULT "+JSON.stringify(o)); return o;
})();
