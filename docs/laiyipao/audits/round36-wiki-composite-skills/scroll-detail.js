(() => { try {
  const q=(s,r)=>(r||document).querySelector(s);
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const wrap=q('.r36-composite-table-scroll');
  if(!wrap) return JSON.stringify({error:'NO_WRAP'});
  const info=(el)=>el?{tag:el.tagName,cls:String(el.className||'').slice(0,120),sw:el.scrollWidth,cw:el.clientWidth,sl:el.scrollLeft,ox:getComputedStyle(el).overflowX}:null;
  const candidates=[wrap, q('.el-table',wrap), q('.el-table__body-wrapper',wrap), q('.el-scrollbar__wrap',wrap), q('.el-table__header-wrapper',wrap)];
  const before=candidates.map(info);
  // scroll every candidate and see which moves
  const moved=[];
  for(let i=0;i<candidates.length;i++){const el=candidates[i]; if(!el) continue; const b=el.scrollLeft; el.scrollLeft=99999; const a=el.scrollLeft; el.scrollLeft=b; if(a!==b) moved.push({i:i,cls:String(el.className||'').slice(0,80),from:b,to:a});}
  // check last column reachable in header
  const cells=qa('.el-table__header th .cell',wrap).map(e=>(e.innerText||e.textContent||'').trim());
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,before:before,moved:moved,headers:cells,docW:de.scrollWidth,innerW:innerWidth,clientW:de.clientWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()