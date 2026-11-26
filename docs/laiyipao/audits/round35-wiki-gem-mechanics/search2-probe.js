(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const body=q('.wiki-body'); const focusAttr=body?body.getAttribute('data-focus'):null;
  const focusRows=qa('tr.row-focus').map(tr=>txt(tr));
  const tables=qa('.r35-gem-table-scroll .el-table');
  const qt=tables[1]||null; const gt=tables[0]||null;
  const goldRow=qt?qa('.el-table__body tbody tr',qt).find(tr=>txt(tr).indexOf('gold')>=0):null;
  const fengRow=gt?qa('.el-table__body tbody tr',gt).find(tr=>txt(tr).indexOf('锋锐石')>=0):null;
  return JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focusAttr:focusAttr,focusRows:focusRows,inputValue:q('input[type=search]')?.value||'',activeNav:qa('.nav-item.on').map(txt),goldHasFocus:!!goldRow&&goldRow.classList.contains('row-focus'),fengHasFocus:!!fengRow&&fengRow.classList.contains('row-focus'),goldRowText:goldRow?txt(goldRow):null,fengRowText:fengRow?txt(fengRow):null,gemSection:!!q('.r35-gem-mechanics')&&q('.r35-gem-mechanics').offsetParent!==null,badText:/MinValue|MaxValue|SeedGemAffixes|[0-9]+%/.test(document.body.innerText)});
}catch(e){return JSON.stringify({error:String(e)})}})()
