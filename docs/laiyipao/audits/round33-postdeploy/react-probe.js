(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  let table=null;
  for(const t of qa('.el-table')){const sec=t.closest('section');if(sec&&txt(sec).indexOf('反应链一览')>=0){table=t;break;}}
  const headers=table?qa('.el-table__header th .cell',table).map(txt):[];
  const rows=table?qa('.el-table__body tbody tr',table).map(tr=>qa('td .cell',tr).map(txt)):[];
  const texts=rows.map(c=>c.join('|'));
  const corrosion=texts.find(t=>t.indexOf('腐蚀扩散')>=0)||'';
  const armor=texts.find(t=>t.indexOf('破甲')>=0)||'';
  const de=document.documentElement;
  const wiki=txt(q('.wiki'));
  return JSON.stringify({hash:location.hash,tableFound:!!table,headers,rowCount:rows.length,
    corrosionRow:corrosion,armorRow:armor,
    corrosionSpread:corrosion.indexOf('传播 1 层')>=0,
    armorShred:armor.indexOf('150‰')>=0,armorKnock:armor.indexOf('4px')>=0,
    hasImplement:wiki.indexOf('层数传播已实现')>=0,hasBurnCloud:wiki.indexOf('burn_cloud')>=0&&wiki.indexOf('仍未实现')>=0,
    oldWording:wiki.indexOf('没有特殊效果的只剩两条')>=0,hasBad:/NaN|undefined/.test(wiki),
    docW:de.scrollWidth,innerW:innerWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()