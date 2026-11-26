(() => { try {
  const qa=s=>Array.from(document.querySelectorAll(s)); const q=s=>document.querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r39-skin-mechanics");
  return JSON.stringify({hashAfter:location.hash,skinsStillThere:!!sec&&sec.offsetParent!==null,
    rowCount:sec?sec.querySelectorAll(".el-table__body tbody tr").length:0,
    onDashboard:location.hash.indexOf("dashboard")>=0,
    activeNav:qa(".nav-item.on").map(txt),ctaText:txt(q(".r39-ai-cta a",sec||document)),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
