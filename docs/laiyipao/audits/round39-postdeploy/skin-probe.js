(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r39-skin-mechanics");
  const wrap=q(".r39-skin-table-scroll",sec||document);
  const headers=wrap?qa(".el-table__header th .cell",wrap).map(txt):[];
  const rows=wrap?qa(".el-table__body tbody tr",wrap).map(tr=>qa("td .cell",tr).map(txt)):[];
  let scroll=null;
  if(wrap){ const inner=q(".el-scrollbar__wrap",wrap)||wrap; const before=inner.scrollLeft;
    inner.scrollLeft=99999; const after=inner.scrollLeft; inner.scrollLeft=before;
    scroll={cls:String(inner.className||"").slice(0,80),sw:inner.scrollWidth,cw:inner.clientWidth,before:before,after:after,ox:getComputedStyle(inner).overflowX}; }
  const note=q(".r39-ai-note",sec||document);
  const cta=q(".r39-ai-cta a",sec||document);
  const basisEl=q(".r39-ai-basis",sec||document);
  const honEl=q(".r39-ai-honesty",sec||document);
  const focusRow=q(".r39-skin-mechanics tr.row-focus");
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,
    sectionVisible:!!sec&&sec.offsetParent!==null,
    skinToc:qa(".nav-item").map(txt).filter(t=>t.indexOf("皮肤")>=0).join("|"),
    headers:headers,rows:rows,scroll:scroll,
    conclusion:txt(q(".r39-ai-conclusion",sec||document)),
    basis:txt(basisEl),basisAttr:basisEl?basisEl.getAttribute("data-ai-basis"):null,
    honesty:txt(honEl),honestyAttr:honEl?honEl.getAttribute("data-ai-honesty"):null,
    ctaText:txt(q(".r39-ai-cta",sec||document)),ctaHref:cta?cta.getAttribute("href"):null,
    ctaVisible:!!cta&&cta.offsetParent!==null,
    focusRowFocus:!!focusRow,focusRowText:focusRow?txt(focusRow).slice(0,120):null,
    docW:de.scrollWidth,innerW:innerWidth,
    bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
