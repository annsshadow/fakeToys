(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const vis=e=>!!e&&e.offsetParent!==null;
  const secs=qa("section.wiki-section").filter(vis);
  const find=re=>secs.find(s=>re.test(txt(s)));
  const tableRows=s=>s?qa(".el-table__body tbody tr",s).length:null;
  const basics=find(/反应链一览/), enemies=find(/敌人抗性图鉴/), skills=find(/技能表/), gems=find(/宝石图鉴/),
    skins=find(/皮肤图鉴/), rules=find(/掉落|数值规则|分数与星级/), mastery=find(/专精树/), levels=find(/章节一览|关卡一览/);
  const reactionRows=tableRows(basics&&q("#reactions",basics));
  const reactionText=basics?txt(basics):"";
  const skillTable=skills&&q('[id="skill-table"] .el-table, .el-table',skills);
  const composite=skills&&q("#composite",skills);
  const recipe=skills&&q(".r38-recipe-mechanics",skills);
  const gemTables=gems?qa(".el-table",gems):[];
  const drop=rules&&q(".r39-drop-table",rules);
  const chapter=levels&&q(".r41-chapter-table-scroll",levels);
  const levelTables=levels?qa(".el-table",levels):[];
  const masteryText=mastery?txt(mastery):"";
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,reaction:reactionRows,has150:reactionText.indexOf("150‰")>=0,has4px:reactionText.indexOf("4px")>=0,
    skills:tableRows(skillTable),composite:tableRows(composite),recipes:tableRows(recipe),enemies:tableRows(enemies),
    levels:levelTables.length?tableRows(levelTables[levelTables.length-1]):null,
    gems:gemTables.length?tableRows(gemTables[0]):null,qualities:gemTables.length>1?tableRows(gemTables[1]):null,
    skins:tableRows(skins),rating:tableRows(find(/构筑评分/)),drop:tableRows(drop),chapter:tableRows(chapter),
    masteryNodes:(masteryText.match(/已投入/g)||[]).length,masteryTableRows:mastery?qa(".el-table__body tbody tr",mastery).length:null,
    dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),docW:de.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()