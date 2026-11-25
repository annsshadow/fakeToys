(() => { try {
 const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s);
 const txt=e=>e?(e.innerText||e.textContent||'').trim():''; const cells=row=>qa(':scope > td > .cell',row).map(txt);
 const sec=q('.r34-level-mechanics'); const scroll=q('.r34-level-table-scroll');
 const wrap=q(':scope > .el-table__body-wrapper .el-scrollbar__wrap',scroll);
 const header=q(':scope > .el-table__header-wrapper',scroll);
 const headers=header?qa('th .cell',header).map(txt):[];
 const rows=qa('.el-table__body tbody tr',scroll).map(cells);
 const note=q('.r42-level-chapter-note'); const de=document.documentElement;
 return JSON.stringify({hash:location.hash,visible:!!sec&&sec.offsetParent!==null,headers,headerCount:headers.length,rows,rowCount:rows.length,
  ai:txt(note&&q('p strong',note))+txt(note&&q('p',note)).replace(txt(note&&q('p strong',note)),''),
  basis:note&&q('.r42-ai-basis',note)?txt(note.querySelector('.r42-ai-basis')):null,
  honesty:note&&q('.r42-ai-honesty',note)?txt(note.querySelector('.r42-ai-honesty')):null,
  ctaText:note&&q('.r42-ai-cta a',note)?txt(note.querySelector('.r42-ai-cta a')):null,
  ctaHref:note&&q('.r42-ai-cta a',note)?q('.r42-ai-cta a',note).getAttribute('href'):null,
  scroll:{sw:wrap?wrap.scrollWidth:null,cw:wrap?wrap.clientWidth:null,sl:wrap?wrap.scrollLeft:null,ox:wrap?getComputedStyle(wrap).overflowX:null},
  page:{docW:de.scrollWidth,innerW:innerWidth},dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()