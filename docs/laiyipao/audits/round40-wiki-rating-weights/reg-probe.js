(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
  const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const visible=e=>!!e&&e.offsetParent!==null;
  const visSections=qa("section.wiki-section").filter(visible);
  const secInfo=visSections.map(s=>({cls:String(s.className||"").slice(0,60),
    headings:qa("h4",s).map(txt),
    tables:qa(".el-table",s).map(t=>({rows:qa(".el-table__body tbody tr",t).length,
      firstRow:(()=>{const tr=q(".el-table__body tbody tr",t);return tr?qa("td .cell",tr).map(txt).slice(0,4):[]})()}))}));
  const reactTable=visSections.length?q(".el-table",visSections[0]):null;
  const rr=reactTable?qa(".el-table__body tbody tr",reactTable).map(tr=>qa("td .cell",tr).map(txt)):[];
  const corrosion=rr.find(r=>r.join("|").indexOf("腐蚀扩散")>=0)||[];
  const armor=rr.find(r=>r.join("|").indexOf("破甲")>=0)||[];
  const comp=q("#composite"); const recipe=q(".r38-recipe-mechanics");
  const skin=q(".r39-skin-mechanics"); const drop=q(".r39-drop-table");
  const rating=q(".r40-rating-mechanics");
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,onTabs:qa(".nav-item.on").map(txt),sections:secInfo,
    reactionRows:rr.length,corrosionSpread:corrosion.join("|").indexOf("传播 1 层")>=0,
    armorShred:armor.join("|").indexOf("150‰")>=0,armorKnock:armor.join("|").indexOf("4px")>=0,
    compositeVisible:!!comp&&visible(comp),compositeRows:comp&&visible(comp)?qa(".el-table__body tbody tr",comp).length:0,
    recipeVisible:!!recipe&&visible(recipe),recipeRows:recipe&&visible(recipe)?qa(".el-table__body tbody tr",recipe).length:0,
    skinVisible:!!skin&&visible(skin),skinRows:skin&&visible(skin)?qa(".el-table__body tbody tr",skin).length:0,
    dropVisible:!!drop&&visible(drop),dropRows:drop&&visible(drop)?qa(".el-table__body tbody tr",drop).length:0,
    ratingVisible:!!rating&&visible(rating),
    ratingRows:rating&&visible(rating)?qa(".el-table__body tbody tr",rating).map(tr=>qa("td .cell",tr).map(txt)):[],
    docW:de.scrollWidth,innerW:innerWidth,
    bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
