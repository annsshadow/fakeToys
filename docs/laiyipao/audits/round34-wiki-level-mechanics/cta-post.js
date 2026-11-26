(() => { try {
  const q=(s)=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  return JSON.stringify({hashAfter:location.hash,levelStillThere:!!q('.r34-level-mechanics .el-table'),rowCount:document.querySelectorAll('.r34-level-mechanics .el-table__body tbody tr').length,compositeVisible:!!q('#composite'),dashboard:location.hash.includes('/admin/dashboard'),activeNav:Array.from(document.querySelectorAll('.nav-item.on')).map(txt)});
}catch(e){return JSON.stringify({error:String(e)})}})()
