(() => { try {
 const qa=s=>Array.from(document.querySelectorAll(s)); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
 const q=s=>document.querySelector(s);
 const panel=q(".search-panel");
 const hits=qa(".search-hit").map(h=>({kind:txt(q(".hit-kind",h)),title:txt(q(".hit-title",h)),meta:txt(q(".hit-meta",h))}));
 return JSON.stringify({hash:location.hash,query:q(".search-input")?q(".search-input").value:null,panelVisible:!!panel&&panel.offsetParent!==null,hits:hits,totalLine:txt(q(".search-more")),empty:txt(q(".search-empty"))});
}catch(e){return JSON.stringify({error:String(e)})}})()
