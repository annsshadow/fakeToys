(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const sel=q('.r34-level-mechanics .el-select');
  const items=qa('.el-select-dropdown__item').map(txt);
  return JSON.stringify({selFound:!!sel,selText:txt(sel),itemsVisible:items.length,items,opts:qa('.r34-level-mechanics .el-select-dropdown__item').map(txt)});
}catch(e){return JSON.stringify({error:String(e)})}})()