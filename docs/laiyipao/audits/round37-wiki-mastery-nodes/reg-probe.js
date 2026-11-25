(() => { try {
  const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sectionWith=(t)=>qa("section").find(s=>txt(s).includes(t));
  const countRows=(el)=>{const t=el?q(".el-table",el):null; return t?qa(".el-table__body tbody tr",t).length:0;};
  const react=sectionWith("反应链一览"); const rt=react?q(".el-table",react):null;
  const rr=rt?qa(".el-table__body tbody tr",rt).map(tr=>qa("td .cell",tr).map(txt)):[];
  const corrosion=rr.find(r=>r.join("|").includes("腐蚀扩散"))||[];
  const armor=rr.find(r=>r.join("|").includes("破甲"))||[];
  const composite=q("#composite"); const mastery=q(".r37-mastery-mechanics");
  const tabCounts={basics:countRows(sectionWith("基础规则")),skills:countRows(sectionWith("技能图鉴")),composite:composite?qa(".el-table__body tbody tr",composite).length:0,enemies:countRows(sectionWith("敌人抗性图鉴")),levels:countRows(sectionWith("关卡一览")),gems:countRows(sectionWith("宝石图鉴")),mastery:mastery?qa(".el-table__body tbody tr",mastery).length:0};
  const de=document.documentElement;
  return JSON.stringify({hash:location.hash,reactionCount:rr.length,corrosionSpread:corrosion.join("|").includes("传播 1 层"),armorShred:armor.join("|").includes("150‰"),armorKnock:armor.join("|").includes("4px"),tabCounts:tabCounts,docW:de.scrollWidth,innerW:innerWidth,bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText)});
}catch(e){return JSON.stringify({error:String(e)})}})()