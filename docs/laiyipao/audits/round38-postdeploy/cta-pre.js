(() => { try {
 const q=s=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
 const sec=q(".r38-recipe-mechanics"); const cta=q(".r38-ai-cta a",sec||document);
 return JSON.stringify({hashBefore:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,rowCount:sec?sec.querySelectorAll(".el-table__body tbody tr").length:0,ctaText:txt(cta),ctaHref:cta?cta.getAttribute("href"):null});
}catch(e){return JSON.stringify({error:String(e)})}})()
