(() => { try {
  const q=(s)=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r37-mastery-mechanics");
  return JSON.stringify({hashAfter:location.hash,masteryStillThere:!!sec&&sec.offsetParent!==null,rowCount:sec?sec.querySelectorAll(".el-table__body tbody tr").length:0,dashboard:location.hash.includes("/admin/dashboard"),activeNav:Array.from(document.querySelectorAll(".nav-item.on")).map(txt),ctaText:txt(q(".r37-ai-cta a",sec||document))});
}catch(e){return JSON.stringify({error:String(e)})}})()