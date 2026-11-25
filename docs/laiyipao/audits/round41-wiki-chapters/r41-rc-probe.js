(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)), q=(s,r)=>(r||document).querySelector(s), txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const basics=q("section.wiki-section");
  const reactTable=basics?qa(".el-table",basics)[0]:null;
  const composite=q("#composite");
  const compTables=composite?qa(".el-table",composite):[];
  return JSON.stringify({hash:location.hash,
    reactionRows:reactTable?qa(".el-table__body tbody tr",reactTable).length:null,
    reactionHas150:txt(basics).indexOf("150‰")>=0, reactionHas4px:txt(basics).indexOf("4px")>=0,
    compositeTables:compTables.length, compositeTableRows:compTables.map(t=>qa(".el-table__body tbody tr",t).length),
    compositeText:composite?txt(composite).slice(0,180):null, err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()