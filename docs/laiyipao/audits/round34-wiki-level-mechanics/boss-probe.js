(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const rows=qa('.r34-level-mechanics tbody tr').map(tr=>qa('td',tr).map(txt));
  const boss=rows.filter(c=>c.length>=13 && c[9]==='是');
  const armor=rows.filter(c=>c.length>=13 && /‰/.test(c[11]));
  return JSON.stringify({rows:rows.length,first:rows[0]&&rows[0][0],last:rows[rows.length-1]&&rows[rows.length-1][0],boss:boss.map(c=>c.slice(0,13)),armor:armor.slice(0,3).map(c=>[c[0],c[11]]),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
