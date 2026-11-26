(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r38-recipe-mechanics");
  const b=q(".r38-recipe-table-scroll",sec||document);
  const hdr=b?qa(".el-table__header th .cell",b).map(txt):[];
  const rows=b?qa(".el-table__body tbody tr",b).map(tr=>qa("td .cell",tr).map(txt)):[];
  const r36=q(".r36-composite-mechanics");
  const wrapInfo=(()=>{
   if(!b) return [];
   const inner=q(".el-scrollbar__wrap",b)||b;
   const before=inner.scrollLeft; inner.scrollLeft=99999; const after=inner.scrollLeft; inner.scrollLeft=before;
   return [{cls:String(inner.className||"").slice(0,80),sw:inner.scrollWidth,cw:inner.clientWidth,before:before,after:after,ox:getComputedStyle(inner).overflowX,headers:hdr}];
  })();
  const note=q(".r38-ai-note",sec||document); const cta=q(".r38-ai-cta a",sec||document);
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,headers:hdr,rows:rows,wrap:wrapInfo,
   r36Visible:!!r36&&r36.offsetParent!==null,r36Rows:r36?r36.querySelectorAll(".el-table__body tbody tr").length:0,
   aiNote:txt(note),conclusion:txt(q(".r38-ai-conclusion",sec||document)),basis:txt(q(".r38-ai-basis",sec||document)),
   honesty:txt(q(".r38-ai-honesty",sec||document)),ctaText:txt(q(".r38-ai-cta",sec||document)),
   ctaHref:cta?cta.getAttribute("href"):null,ctaVisible:!!cta&&cta.offsetParent!==null,
   docW:de.scrollWidth,innerW:innerWidth,badText:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
