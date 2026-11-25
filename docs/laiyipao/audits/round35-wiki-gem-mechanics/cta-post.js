(() => { try { const q=(s)=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
 return JSON.stringify({hashAfter:location.hash,gemStillThere:!!q('.r35-gem-mechanics .el-table'),rowTotal:document.querySelectorAll('.r35-gem-mechanics .el-table__body tbody tr').length,dashboard:location.hash.includes('/admin/dashboard'),activeNav:Array.from(document.querySelectorAll('.nav-item.on')).map(txt)});
}catch(e){return JSON.stringify({error:String(e)})}})()
