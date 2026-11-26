(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r40-rating-mechanics");
  const wrap=q(".r40-rating-table-scroll",sec||document);
  const headers=wrap?qa(".el-table__header th .cell",wrap).map(txt):[];
  const rows=wrap?qa(".el-table__body tbody tr",wrap).map(tr=>qa("td .cell",tr).map(txt)):[];
  let scroll=null;
  if(wrap){ const inner=q(".el-scrollbar__wrap",wrap)||wrap; const before=inner.scrollLeft;
    inner.scrollLeft=99999; const after=inner.scrollLeft; inner.scrollLeft=before;
    scroll={cls:String(inner.className||"").slice(0,80),sw:inner.scrollWidth,cw:inner.clientWidth,before:before,after:after,ox:getComputedStyle(inner).overflowX}; }
  const cta=q(".r40-ai-cta a",sec||document);
  const basisEl=q(".r40-ai-basis",sec||document);
  const honEl=q(".r40-ai-honesty",sec||document);
  const secText=sec?txt(sec):"";
  const focusRow=q(".r40-rating-mechanics tr.row-focus");
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,
    sectionVisible:!!sec&&sec.offsetParent!==null,
    tocRating:qa(".nav-item,.nav-sub-item").map(txt).filter(t=>t.indexOf("构筑评分")>=0).join("|"),
    headers:headers,rows:rows,scroll:scroll,
    conclusion:txt(q(".r40-ai-conclusion",sec||document)),
    basis:txt(basisEl),basisTitle:basisEl?basisEl.getAttribute("title"):null,
    honesty:txt(honEl),honestyAttr:honEl?honEl.getAttribute("data-r40-honesty"):null,
    ctaText:txt(q(".r40-ai-cta",sec||document)),ctaHref:cta?cta.getAttribute("href"):null,
    ctaVisible:!!cta&&cta.offsetParent!==null,
    focusRowFocus:!!focusRow,focusRowText:focusRow?txt(focusRow).slice(0,120):null,
    secHasPct:/%|‰/.test(secText),
    docW:de.scrollWidth,innerW:innerWidth,
    bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()