(() => { try {
  const q=s=>document.querySelector(s);
  return JSON.stringify({hashAfter:location.hash,enemyStillThere:!!q('.r33-enemy-mechanics'),rowCount:document.querySelectorAll('.r33-enemy-mechanics .el-table__body tbody tr').length,dashboard:location.hash.includes('dashboard')||!!q('.dashboard'),navActive:[...document.querySelectorAll('.nav-item.on')].map(x=>x.textContent.trim()),scrollY:Math.round(scrollY)});
}catch(e){return JSON.stringify({error:String(e)})}})()