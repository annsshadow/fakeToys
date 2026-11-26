(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const sec=q('.r35-gem-mechanics'); const tables=qa('.r35-gem-table-scroll .el-table');
  const t=(i)=>{const tb=tables[i]; if(!tb) return null; return {headers:qa('.el-table__header th .cell',tb).map(txt).filter(Boolean),rows:qa('.el-table__body tbody tr',tb).map(tr=>qa('td .cell',tr).map(txt))};};
  const wraps=qa('.r35-gem-table-scroll'); const w=wraps[0]; const before=w?w.scrollLeft:0; if(w) w.scrollLeft=w.scrollWidth; const after=w?w.scrollLeft:0;
  const w2=wraps[1]; const before2=w2?w2.scrollLeft:0; if(w2) w2.scrollLeft=w2.scrollWidth; const after2=w2?w2.scrollLeft:0;
  const de=document.documentElement;
  const note=q('.r35-affix-note'); const cta=q('[data-r35-cta]');
  return JSON.stringify({hash:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,nav:qa('.nav-item.on').map(txt),
   gem:t(0),quality:t(1),
   aiNote:txt(note),conclusion:txt(q('.r35-ai-conclusion')),basis:txt(q('.r35-ai-basis')),ctaText:txt(cta),ctaHref:cta?cta.getAttribute('href'):null,ctaVisible:!!cta&&cta.offsetParent!==null,honesty:txt(q('.r35-ai-honesty')),
   docW:de.scrollWidth,clientW:de.clientWidth,innerW:innerWidth,
   wrap1:{sw:w?w.scrollWidth:0,cw:w?w.clientWidth:0,before:before,after:after,overflow:w?getComputedStyle(w).overflowX:null},
   wrap2:{sw:w2?w2.scrollWidth:0,cw:w2?w2.clientWidth:0,before:before2,after:after2,overflow:w2?getComputedStyle(w2).overflowX:null},
   badText:/MinValue|MaxValue|SeedGemAffixes|[0-9]+%/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
