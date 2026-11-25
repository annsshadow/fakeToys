(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||'').trim():'';
  const reactTable=q('#reaction-table');
  const skillTable=q('#skill-table');
  const reactRows=reactTable?qa('.el-table__body tbody tr',reactTable):[];
  const skillRows=skillTable?qa('.el-table__body tbody tr',skillTable):[];
  const reactTexts=reactRows.map(tr=>qa('td .cell',tr).map(txt).join('|'));
  const corrosion=reactTexts.find(t=>t.includes('腐蚀扩散'))||'';
  const armor=reactTexts.find(t=>t.includes('破甲'))||'';
  const skillTexts=skillRows.map(tr=>qa('td .cell',tr).map(txt).join('|'));
  const sk3=skillTexts.find(t=>/(^|\|)3(\|)/.test(t))||'';
  const de=document.documentElement;
  return JSON.stringify({
    hash:location.hash,
    reactionCount:reactRows.length,skillCount:skillRows.length,
    corrosionRow:corrosion,armorRow:armor,
    corrosionSpread:corrosion.includes('传播 1 层'),
    armorShred:armor.includes('150‰'),armorKnock:armor.includes('4px'),
    sk3Row:sk3,sk3Active:sk3.includes('主动'),sk3Cd:sk3.includes('2秒'),
    hasBad:/NaN|undefined|\[object Object\]|\{total\}/.test(document.body.innerText),
    docW:de.scrollWidth,innerW:innerWidth
  });
}catch(e){return JSON.stringify({error:String(e)})}})()