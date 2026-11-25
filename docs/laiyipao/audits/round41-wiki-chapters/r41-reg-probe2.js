(() => { try {
  const q=(s,r)=>(r||document).querySelector(s), qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const vis=s=>{const e=q(s);return e&&e.offsetParent!==null?e:null;};
  const tables=s=>{const e=vis(s);return e?qa(".el-table__body tbody tr",e).length:-1;};
  const sections=qa("section.wiki-section").filter(e=>e.offsetParent!==null);
  const allTxt=sections.map(txt).join("\n");
  const chapter=vis(".r41-chapter-table-scroll");
  const chapterRows=chapter?qa(".el-table__body tbody tr",chapter).length:-1;
  const masteryBlocks=sections.map(s=>txt(s)).filter(t=>t.indexOf("专精")>=0).join("\n");
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,active:qa(".nav-item.on").map(txt),
    reaction:qa(".el-table__body tbody tr",vis("section.wiki-section")||document).length,
    reaction150:allTxt.indexOf("150‰")>=0,reaction4px:allTxt.indexOf("4px")>=0,
    skills:tables("#skill-table"), composite:tables("#composite"), recipes:tables(".r38-recipe-mechanics"),
    enemies:tables(".wiki-section .el-table"), levels:qa(".el-table__body tbody tr",sections.find(s=>txt(s).indexOf("关卡一览")>=0)||document).length,
    gems:tables(".r31-gem-mechanics"), drop:tables(".r39-drop-table"),
    skins:tables(".r39-skin-mechanics"), rating:tables(".r40-rating-mechanics"), chapter:chapterRows,
    masteryNodes:(masteryBlocks.match(/已投入/g)||[]).length,
    hasEmptyText:/NaN|undefined|\[object Object\]/.test(allTxt),docW:de.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()