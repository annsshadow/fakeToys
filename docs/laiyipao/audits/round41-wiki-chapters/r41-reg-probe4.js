(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const rows=sel=>{const e=q(sel);return e?qa(".el-table__body tbody tr",e).length:null;};
  const basics=q("section.wiki-section"), basicText=basics?txt(basics):"";
  const enemy=q(".r33-enemy-mechanics");
  const skill=q("#skill-table"), composite=q("#composite"), recipe=q(".r38-recipe-mechanics");
  const gem=q(".r35-gem-mechanics");
  const skin=q(".r39-skin-mechanics"), drop=q(".r39-drop-table");
  const mastery=q(".r37-mastery-mechanics"), level=q(".r34-level-mechanics");
  const chapter=q(".r41-chapter-table-scroll"), levelTables=level?qa(".el-table",level):[];
  const gemTables=gem?qa(".el-table",gem):[];
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,reaction:rows("#reactions")>=0?rows("#reactions"):qa(".el-table__body tbody tr",q("#reactions")||document).length,
    has150:basicText.indexOf("150‰")>=0||txt(document.body).indexOf("150‰")>=0,has4px:basicText.indexOf("4px")>=0||txt(document.body).indexOf("4px")>=0,
    skills:rows("#skill-table"),composite:rows("#composite"),recipes:rows(".r38-recipe-mechanics"),enemies:rows(".r33-enemy-mechanics"),
    levels:levelTables.length>1?qa(".el-table__body tbody tr",levelTables[levelTables.length-1]).length:null,
    gems:gemTables.length?qa(".el-table__body tbody tr",gemTables[0]).length:null,
    qualities:gemTables.length>1?qa(".el-table__body tbody tr",gemTables[1]).length:null,
    skins:rows(".r39-skin-mechanics"),rating:rows(".r40-rating-mechanics"),drop:rows(".r39-drop-table"),
    chapter:chapter?qa(".el-table__body tbody tr",chapter).length:null,
    masteryRows:mastery?qa(".el-table__body tbody tr",mastery).length:null,
    dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),docW:de.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()