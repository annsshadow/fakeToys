(() => { try {
 const q=(s,r)=>(r||document).querySelector(s); const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
 const scroll=q('.r34-level-table-scroll'); const rows=qa('.el-table__body tbody tr',scroll); const cells=r=>qa(':scope > td > .cell',r).map(e=>(e.innerText||e.textContent||'').trim());
 const de=document.documentElement;
 return JSON.stringify({hash:location.hash,visible:!!scroll&&scroll.offsetParent!==null,rowCount:rows.length,first:rows[0]?cells(rows[0]):null,last:rows[rows.length-1]?cells(rows[rows.length-1]):null,
  selected:q('.el-select')?((q('.el-select input')||{}).value||null):null,docW:de.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()