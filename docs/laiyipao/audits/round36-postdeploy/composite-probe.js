(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const sec=q('#composite'); const wrap=q('.r36-composite-table-scroll'); const table=wrap?q('.el-table',wrap):null;
  const headers=table?qa('.el-table__header th .cell',table).map(txt).filter(Boolean):[];
  const rows=table?qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt)):[];
  const before=wrap?wrap.scrollLeft:0; if(wrap) wrap.scrollLeft=wrap.scrollWidth; const after=wrap?wrap.scrollLeft:0;
  const de=document.documentElement;
  const note=q('.r36-ai-note'); const cta=q('.r36-ai-cta a');
  return JSON.stringify({hash:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,nav:qa('.nav-item.on').map(txt),
   headers:headers,rows:rows,
   aiNote:txt(note),conclusion:txt(q('.r36-ai-conclusion')),basis:txt(q('.r36-ai-basis')),ctaText:txt(q('.r36-ai-cta')),ctaHref:cta?cta.getAttribute('href'):null,ctaVisible:!!cta&&cta.offsetParent!==null,honesty:txt(q('.r36-ai-honesty')),
   docW:de.scrollWidth,clientW:de.clientWidth,innerW:innerWidth,
   wrap:{sw:wrap?wrap.scrollWidth:0,cw:wrap?wrap.clientWidth:0,before:before,after:after,overflow:wrap?getComputedStyle(wrap).overflowX:null},
   badText:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
