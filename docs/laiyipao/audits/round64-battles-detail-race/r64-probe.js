(function(){
  var rows = [].slice.call(document.querySelectorAll(".battle-row"));
  var expanded = rows.filter(function(r){ return r.className.indexOf("expanded") >= 0; })[0] || null;
  var det = expanded ? expanded.querySelector(".battle-detail") : null;
  function txt(e){ return e ? (e.innerText||e.textContent||"").replace(/\s+/g," ").trim() : ""; }
  var kv = det ? [].slice.call(det.querySelectorAll(".kv")).map(txt) : [];
  var hashEl = det ? det.querySelector(".hash") : null;
  var growth = det ? det.querySelector(".r10-growth") : null;
  var hashesEl = growth ? growth.querySelector(".r10-hashes") : null;
  var r14 = det ? det.querySelector(".r14-advice") : null;
  var cta = r14 ? (r14.querySelector("#r14-advice-action-btn") || r14.querySelector(".r14-advice-cta")) : null;
  var ctl = window.__raceCtl || {log:[]};
  return JSON.stringify({
    href: location.href, hash: location.hash,
    rowCount: rows.length,
    expandedHead: expanded ? txt(expanded.querySelector(".battle-head")) : "",
    detailHash: hashEl ? hashEl.textContent.trim() : "",
    kv: kv,
    growthText: txt(growth),
    growthHashes: hashesEl ? hashesEl.textContent.trim() : "",
    r14Text: txt(r14),
    ctaText: cta ? cta.textContent.trim() : "",
    raceLog: ctl.log || [],
    bodyLen: (document.body.innerText||"").length
  });
})()
