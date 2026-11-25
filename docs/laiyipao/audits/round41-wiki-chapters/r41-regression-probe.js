(() => { try {
  const q=(s,r)=>(r||document).querySelector(s), qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const rows=sel=>{const t=q(sel);return t?qa(".el-table__body tbody tr",t).map(tr=>qa("td .cell",tr).map(txt)):[];};
  const rating=q(".r40-rating-mechanics"), react=q(".r15-reaction-mechanics")||qa("section.wiki-section").find(s=>qa("h4",s).some(h=>txt(h).indexOf("反应")>=0));
  const reactionRows=react?qa(".el-table__body tbody tr",react).length:0;
  const reactionText=react?txt(react):"";
  const skills=rows(".r23-skill-mechanics .el-table"), composite=rows("#composite .el-table"), recipes=rows(".r38-recipe-mechanics .el-table");
  const enemies=rows(".r22-enemy-mechanics .el-table"), gems=rows(".r31-gem-mechanics .el-table"), qualities=rows(".r31-gem-mechanics .el-table:nth-of-type(2)");
  const skins=rows(".r39-skin-mechanics .el-table"), ratingRows=rows(".r40-rating-mechanics .el-table");
  const mastery=qa("section.wiki-section").filter(s=>qa("h4",s).some(h=>txt(h).indexOf("专精")>=0)).reduce((n,s)=>n+qa(".el-table__body tbody tr",s).length,0);
  const levels=qa("section.wiki-section").filter(s=>qa("h4",s).some(h=>txt(h).indexOf("关卡")>=0)).reduce((n,s)=>n+qa(".el-table__body tbody tr",s).length,0);
  const dropRows=rows(".r39-drop-table .el-table").length;
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash, reactionRows:reactionRows, has150:reactionText.indexOf("150‰")>=0, has4px:reactionText.indexOf("4px")>=0,
    skills:skills.length, composite:composite.length, recipes:recipes.length, enemies:enemies.length, levels:levels, gems:gems.length,
    gemQualities:qualities.length, mastery:mastery, skins:skins.length, ratingRows:ratingRows.length, drop:dropRows,
    ratingCells:ratingRows, chapterRows:rows(".r41-chapter-table-scroll .el-table").length,
    docW:de.scrollWidth,innerW:innerWidth,bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()