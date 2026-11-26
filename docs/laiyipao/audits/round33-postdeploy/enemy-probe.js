(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const sec=q('.r33-enemy-mechanics');
  const table=q('.r33-enemy-table-scroll .el-table');
  const headers=qa('.el-table__header th .cell',table).map(txt);
  const rows=qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt));
  const ai=q('.r33-ai-note');
  const cta=q('.r33-ai-cta a');
  const wrap=q('.r33-enemy-table-scroll .el-scrollbar__wrap');
  const de=document.documentElement;
  const before=wrap?wrap.scrollLeft:0; if(wrap) wrap.scrollLeft=wrap.scrollWidth; const after=wrap?wrap.scrollLeft:0;
  return JSON.stringify({
    url:location.href,hash:location.hash,sectionVisible:!!sec,headers,rowCount:rows.length,rows,
    aiText:ai?txt(ai):'',conclusion:txt(q('.r33-ai-conclusion')),basis:txt(q('.r33-ai-basis')),
    ctaText:cta?txt(cta):'',ctaHref:cta?(cta.getAttribute('href')||''):'',honesty:txt(q('.r33-ai-honesty')),
    hasBad:/NaN|undefined|\[object Object\]|\{total\}|\{boss\}|\{fly\}|\{burrow\}|\{armor\}/.test(document.body.innerText),
    docW:de.scrollWidth,clientW:de.clientWidth,innerW:innerWidth,bodyW:document.body.scrollWidth,
    wrapSw:wrap?wrap.scrollWidth:0,wrapCw:wrap?wrap.clientWidth:0,wrapBefore:before,wrapAfter:after,
    lastHeader:txt(qa('.el-table__header th .cell',table).pop()),
    scrolledToEnd:after>=((wrap?wrap.scrollWidth:0)-(wrap?wrap.clientWidth:0)-2),
    activeNav:qa('.nav-item.on').map(txt)
  });
}catch(e){return JSON.stringify({error:String(e)})}})()