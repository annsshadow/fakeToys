(() => { try {
  const qa=s=>Array.from(document.querySelectorAll(s)); const q=s=>document.querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const panel=q(".search-panel");
  const hits=qa(".search-hit").map(h=>({kind:txt(q(".hit-kind",h)),title:txt(q(".hit-title",h)),meta:txt(q(".hit-meta",h))}));
  const focusRow=q(".r39-skin-mechanics tr.row-focus");
  return JSON.stringify({hash:location.hash,query:q(".search-input")?q(".search-input").value:null,
    panelVisible:!!panel&&panel.offsetParent!==null,hits:hits,
    skinRowFocus:!!focusRow,focusText:focusRow?txt(focusRow).slice(0,120):null,
    totalLine:txt(q(".search-more")),empty:txt(q(".search-empty")),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()
