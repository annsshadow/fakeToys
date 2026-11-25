(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const table=q('#skill-table');
  const rows=table?qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt)):[];
  const texts=rows.map(c=>c.join('|'));
  const sk3=texts.find(t=>/(^|\|)3(\|)/.test(t))||'';
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,tableFound:!!table,rowCount:rows.length,sk3Row:sk3,
    sk3Active:sk3.indexOf('主动')>=0,sk3Cd:sk3.indexOf('2秒')>=0,
    docW:de.scrollWidth,innerW:innerWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()