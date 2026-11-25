(() => { try {
  const q=(s,r)=>(r||document).querySelector(s); const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const sec=q(".r37-mastery-mechanics"); const blocks=qa(".r37-mastery-table-scroll",sec||document);
  if(!blocks.length) return JSON.stringify({error:"NO_BLOCKS"});
  const info=(el)=>el?{tag:el.tagName,cls:String(el.className||"").slice(0,100),sw:el.scrollWidth,cw:el.clientWidth,sl:el.scrollLeft,ox:getComputedStyle(el).overflowX}:null;
  const out=[];
  for(const b of blocks){
    const candidates=[b,q(".el-table",b),q(".el-table__body-wrapper",b),q(".el-scrollbar__wrap",b),q(".el-table__header-wrapper",b)];
    const before=candidates.map(info); const moved=[];
    for(let i=0;i<candidates.length;i++){const el=candidates[i]; if(!el) continue; const x=el.scrollLeft; el.scrollLeft=99999; const y=el.scrollLeft; el.scrollLeft=x; if(y!==x) moved.push({i:i,cls:String(el.className||"").slice(0,80),from:x,to:y});}
    const ths=qa(".el-table__header th .cell",b).map(e=>(e.innerText||e.textContent||"").trim());
    out.push({before:before,moved:moved,headers:ths});
  }
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,blocks:out.length,detail:out,docW:de.scrollWidth,innerW:innerWidth,clientW:de.clientWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()