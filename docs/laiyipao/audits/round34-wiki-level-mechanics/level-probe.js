(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const sec=q('.r34-level-mechanics');
  const table=q('.r34-level-table-scroll .el-table');
  const headers=qa('.el-table__header th .cell',table).map(txt);
  const rows=qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt));
  const ai=q('.r34-ai-note');
  const cta=q('[data-r34-cta]');
  const wrap=q('.r34-level-table-scroll .el-scrollbar__wrap');
  const de=document.documentElement;
  const select=q('.el-select',sec);
  const chapter=select?txt(q('.el-select__selected-item',select))||txt(q('input',select)):'';
  const before=wrap?wrap.scrollLeft:0; if(wrap) wrap.scrollLeft=wrap.scrollWidth; const after=wrap?wrap.scrollLeft:0;
  return JSON.stringify({
    url:location.href,hash:location.hash,sectionVisible:!!sec,headers,rowCount:rows.length,rows,chapter,
    aiText:ai?txt(ai):'',conclusion:txt(q('.r34-ai-conclusion')),basis:txt(q('.r34-ai-basis')),
    ctaText:cta?txt(cta):'',ctaHref:cta?(cta.getAttribute('href')||''):'',ctaVisible:cta?!!(cta.offsetWidth||cta.offsetHeight):false,
    honesty:txt(q('.r34-ai-honesty')),hasConfigBase:headers.some(h=>h.includes('配置基值')),
    hasBad:/NaN|undefined|\[object Object\]|\{total\}|\{boss\}|\{kinds\}/.test(document.body.innerText),
    docW:de.scrollWidth,clientW:de.clientWidth,innerW:innerWidth,bodyW:document.body.scrollWidth,
    wrapSw:wrap?wrap.scrollWidth:0,wrapCw:wrap?wrap.clientWidth:0,wrapBefore:before,wrapAfter:after,
    wrapOverflowX:wrap?getComputedStyle(wrap).overflowX:'',scrollWrapOverflowX:q('.r34-level-table-scroll')?getComputedStyle(q('.r34-level-table-scroll')).overflowX:'',
    lastHeader:txt(qa('.el-table__header th .cell',table).pop()),
    scrolledToEnd:after>=((wrap?wrap.scrollWidth:0)-(wrap?wrap.clientWidth:0)-2),activeNav:qa('.nav-item.on').map(txt)
  });
}catch(e){return JSON.stringify({error:String(e)})}})()
