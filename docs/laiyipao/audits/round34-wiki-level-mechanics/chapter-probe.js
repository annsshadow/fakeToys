(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const table=q('.r34-level-table-scroll .el-table');
  const headers=qa('.el-table__header th .cell',table).map(txt);
  const rows=qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt));
  return JSON.stringify({hash:location.hash,headers,rowCount:rows.length,rows:rows.map(r=>r.slice(0,13)),firstId:rows[0]?rows[0][0]:'',lastId:rows.length?rows[rows.length-1][0]:''});
}catch(e){return JSON.stringify({error:String(e)})}})()
