(() => { try {
  const q=s=>document.querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const vis=e=>!!e&&e.offsetParent!==null;
  return JSON.stringify({hash:location.hash,hashAfter:location.hash,
    onWikiPath:location.hash.indexOf("#/wiki")===0,
    dashboardJump:/dashboard/i.test(location.hash),
    ratingVisible:vis(q(".r40-rating-mechanics")),
    ratingRows:q(".r40-rating-mechanics")?Array.from(q(".r40-rating-mechanics").querySelectorAll(".el-table__body tbody tr")).length:0,
    activeNav:Array.from(document.querySelectorAll(".nav-item.on")).map(txt),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
