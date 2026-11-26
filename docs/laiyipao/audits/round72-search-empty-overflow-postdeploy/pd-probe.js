(() => {
  const de=document.documentElement;
  const out={href:location.href, iw:window.innerWidth, cw:de.clientWidth, sw:de.scrollWidth,
    hOverflow:de.scrollWidth>de.clientWidth+1, bodyLen:document.body?document.body.innerText.length:0,
    badTokens:[/NaN/,/Infinity/,/undefined/,/\[object Object\]/].map(r=>(document.body&&r.test(document.body.innerText))?r.source:null).filter(Boolean),
    emptyTextNodes:[], allClear:[], emptyCount:0, hasKeyword:false, hasImgLiteral:false};
  const els=Array.from(document.querySelectorAll(".search-empty-text"));
  out.emptyTextNodes=els.map(e=>{const cs=getComputedStyle(e);const cl=Array.from(e.querySelectorAll(".clear-search,.link"));return {t:(e.innerText||"").trim().slice(0,430),len:(e.innerText||"").trim().length,cw:e.clientWidth,sw:e.scrollWidth,w:Math.round(e.getBoundingClientRect().width),display:cs.display,wb:cs.wordBreak,maxW:cs.maxWidth};});
  out.allClear=Array.from(document.querySelectorAll(".clear-search,.search-empty-text .link,.search-empty .link")).map(e=>(e.innerText||"").trim());
  out.emptyCount=els.length;
  const t=out.emptyTextNodes.map(x=>x.t).join("");
  out.hasKeyword=t.length>=350 && t.indexOf("Z".repeat(60))>=0;
  out.hasImgLiteral=t.indexOf("<img")>=0;
  console.log("PD_PROBE "+JSON.stringify(out)); return out;
})();
