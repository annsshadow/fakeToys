(() => { try {
 const q=(s,r)=>(r||document).querySelector(s); const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
 const sec=q(".r38-recipe-mechanics"); const b=q(".r38-recipe-table-scroll",sec||document);
 if(!b) return JSON.stringify({error:"NO_BLOCK"});
 const candidates=[b,q(".el-table",b),q(".el-table__body-wrapper",b),q(".el-scrollbar__wrap",b),q(".el-table__header-wrapper",b)];
 const info=el=>el?{tag:el.tagName,cls:String(el.className||"").slice(0,90),sw:el.scrollWidth,cw:el.clientWidth,sl:el.scrollLeft,ox:getComputedStyle(el).overflowX}:null;
 const before=candidates.map(info); const moved=[];
 for(let i=0;i<candidates.length;i++){const el=candidates[i]; if(!el) continue; const x=el.scrollLeft; el.scrollLeft=99999; const y=el.scrollLeft; el.scrollLeft=x; if(y!==x) moved.push({i:i,cls:String(el.className||"").slice(0,80),from:x,to:y});}
 const ths=qa(".el-table__header th .cell",b).map(e=>(e.innerText||e.textContent||"").trim());
 const de=document.documentElement;
 return JSON.stringify({hash:location.hash,before:before,moved:moved,headers:ths,docW:de.scrollWidth,innerW:innerWidth,clientW:de.clientWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()
