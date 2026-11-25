(() => { try {
  const q=(s,r)=>(r||document).querySelector(s);
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const tr=(root,inner)=>{const e=q(root);return e?qa(inner||".el-table__body tbody tr",e).length:null;};
  const basics=q('section.wiki-section'); const bt=basics?txt(basics):"";
  const reactionHost=q("#reactions");
  const reaction=reactionHost?qa(".el-table__body tbody tr",reactionHost.parentElement).length:null;
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,reaction:reaction,
    has150:bt.indexOf("150‰")>=0,has4px:bt.indexOf("4px")>=0,
    skills:tr(".r32-skill-table-scroll"),compositeRows:tr(".r36-composite-mechanics",".r36-composite-table-scroll .el-table__body tbody tr"),recipeRows:tr(".r38-recipe-mechanics",".r38-recipe-table-scroll .el-table__body tbody tr"),
    enemies:tr(".r33-enemy-mechanics"),levels:tr(".r34-level-mechanics",".r34-level-table-scroll .el-table__body tbody tr"),
    gems:tr(".r35-gem-mechanics .r35-gem-table-scroll"),skins:tr(".r39-skin-mechanics"),rating:tr(".r40-rating-mechanics"),
    drop:tr(".r39-drop-table"),chapter:tr(".r41-chapter-table-scroll"),masteryRows:tr(".r37-mastery-mechanics"),
    dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),docW:de.scrollWidth,innerW:innerWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()