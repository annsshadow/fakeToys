(() => { try {
const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)); const q=(s,r)=>(r||document).querySelector(s); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
const visible=(e)=>!!e&&e.offsetParent!==null;
const onTabs=qa(".nav-item.on").map(txt);
const visibleSection=qa("section.wiki-section").filter(visible);
const sectionInfo=visibleSection.map(s=>({cls:String(s.className||"").slice(0,80),text:txt(s).slice(0,12)}));
const findSec=(t)=>visibleSection.find(s=>txt(s).includes(t));
const count=(sec)=>{const t=sec?q(".el-table",sec):null; return t?qa(".el-table__body tbody tr",t).length:0;};
const basicsSec=findSec("为什么「战力」不是最重要的数字")||findSec("反应链一览");
const tableCount=(sec)=>{const ts=sec?qa(".el-table",sec):[]; return ts.map(t=>qa(".el-table__body tbody tr",t).length);};
const reactSec=findSec("反应链一览"); const reactTable=reactSec?q(".el-table",reactSec):null;
const rr=reactTable?qa(".el-table__body tbody tr",reactTable).map(tr=>qa("td .cell",tr).map(txt)):[];
const corrosion=rr.find(r=>r.join("|").includes("腐蚀扩散"))||[]; const armor=rr.find(r=>r.join("|").includes("破甲"))||[];
const masterySec=findSec("每层只能选 2 个节点");
const de=document.documentElement;
return JSON.stringify({hash:location.hash,onTabs:onTabs,visibleSections:sectionInfo,visibleSectionCount:visibleSection.length,reactionCount:rr.length,corrosionSpread:corrosion.join("|").includes("传播 1 层"),armorShred:armor.join("|").includes("150‰"),armorKnock:armor.join("|").includes("4px"),basicsTableCounts:basicsSec?tableCount(basicsSec):[],skillsCount:findSec("技能表")?count(findSec("技能表")):0,compositeCount:count(q("#composite")),masteryCount:masterySec?qa(".el-table__body tbody tr",masterySec).length:0,docW:de.scrollWidth,innerW:innerWidth,bad:/NaN|undefined|\[object Object\]/.test(document.body.innerText)});
}catch(e){return JSON.stringify({error:String(e)})}})()