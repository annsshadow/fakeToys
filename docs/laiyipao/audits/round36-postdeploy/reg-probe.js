(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():"";
  const de=document.documentElement;
  const reactSection=qa("section").find(s=>txt(s).includes("反应链一览"));
  const reactTable=reactSection?q(".el-table",reactSection):null;
  const reactionRows=reactTable?qa(".el-table__body tbody tr",reactTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  const corrosion=reactionRows.find(r=>r.join("|").includes("腐蚀扩散"))||[];
  const armor=reactionRows.find(r=>r.join("|").includes("破甲"))||[];
  const skillTable=q("#skill-table");
  const skillRows=skillTable?qa(".el-table__body tbody tr",skillTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  const enemyTable=q(".r33-enemy-table-scroll .el-table");
  const enemyRows=enemyTable?qa(".el-table__body tbody tr",enemyTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  const levelTable=q(".r34-level-table-scroll .el-table");
  const levelRows=levelTable?qa(".el-table__body tbody tr",levelTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  const gemTable=q(".r35-gem-table-scroll .el-table");
  const gemRows=gemTable?qa(".el-table__body tbody tr",gemTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  return JSON.stringify({hash:location.hash,reactionCount:reactionRows.length,corrosionSpread:corrosion.join("|").includes("传播 1 层"),armorShred:armor.join("|").includes("150‰"),armorKnock:armor.join("|").includes("4px"),skillCount:skillRows.length,enemyCount:enemyRows.length,levelCount:levelRows.length,gemCount:gemRows.length,docW:de.scrollWidth,innerW:innerWidth,bad:/NaN|undefined/.test(document.body.innerText)});
}catch(e){return JSON.stringify({error:String(e)})}})()
