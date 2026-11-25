(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r37-mastery-mechanics");
  const blocks=qa(".r37-mastery-table-scroll",sec||document);
  const headers=blocks.map(b=>qa(".el-table__header th .cell",b).map(txt));
  const rows=[]; const families=[];
  for(const b of blocks){
    const fam=txt(q(".family-name",b));
    const trs=qa(".el-table__body tbody tr",b);
    for(const tr of trs) rows.push(qa("td .cell",tr).map(txt));
    families.push({family:fam,rows:trs.length});
  }
  const de=document.documentElement;
  const wrapInfo=blocks.map((b,i)=>{
    const inner=q(".el-scrollbar__wrap",b); const el=inner||b;
    const before=el.scrollLeft; el.scrollLeft=99999; const after=el.scrollLeft;
    const ths=qa(".el-table__header th",b); const last=ths[ths.length-1];
    const lr=last?last.getBoundingClientRect():null; const er=el.getBoundingClientRect();
    const lastInView=!!lr&&!!er&&lr.right<=er.right+1&&lr.left>=er.left-1;
    el.scrollLeft=before;
    return {i:i,cls:String(el.className||"").slice(0,80),sw:el.scrollWidth,cw:el.clientWidth,before:before,after:after,ox:getComputedStyle(el).overflowX,lastHeader:last?txt(last):null,lastInView:lastInView,headers:headers[i]};
  });
  const note=q(".r37-ai-note",sec||document); const cta=q(".r37-ai-cta a",sec||document);
  return JSON.stringify({hash:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,nav:qa(".nav-item.on").map(txt),families:families,headers:headers,rows:rows,wrap:wrapInfo,aiNote:txt(note),conclusion:txt(q(".r37-ai-conclusion",sec||document)),basis:txt(q(".r37-ai-basis",sec||document)),honesty:txt(q(".r37-ai-honesty",sec||document)),ctaText:txt(q(".r37-ai-cta",sec||document)),ctaHref:cta?cta.getAttribute("href"):null,ctaVisible:!!cta&&cta.offsetParent!==null,docW:de.scrollWidth,innerW:innerWidth,clientW:de.clientWidth,badText:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()