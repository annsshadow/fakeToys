(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const dt=q(".r39-drop-table");
  const headers=dt?qa(".el-table__header th .cell",dt).map(txt):[];
  const rows=dt?qa(".el-table__body tbody tr",dt).map(tr=>qa("td .cell",tr).map(txt)):[];
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,dropVisible:!!dt&&dt.offsetParent!==null,
    headers:headers,rows:rows,dropText:txt(dt),hasPct:/[%‰]/.test(txt(dt)),
    docW:de.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
