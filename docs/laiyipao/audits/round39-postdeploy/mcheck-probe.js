(() => { try {
  const qa=s=>Array.from(document.querySelectorAll(s));
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  return JSON.stringify({hash:location.hash,
    on:qa(".nav-item.on").map(txt),
    vis:qa("section.wiki-section").filter(e=>e.offsetParent!==null).map(e=>String(e.className||"").slice(0,40)),
    masteryTables:qa(".r37-mastery-mechanics .el-table").map(t=>qa(".el-table__body tbody tr",t).length)});
}catch(e){return JSON.stringify({error:String(e)})}})()
