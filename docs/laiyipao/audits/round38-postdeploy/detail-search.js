(() => { try {
 const qa=s=>Array.from(document.querySelectorAll(s)); const q=s=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
 return JSON.stringify({query:txt(q(".search-input")),total:txt(q(".search-more")),hits:qa(".search-hit").map(h=>({kind:txt(q(".hit-kind",h)),title:txt(q(".hit-title",h)),meta:txt(q(".hit-meta",h)),html:h.outerHTML.slice(0,260)}))});
}catch(e){return JSON.stringify({error:String(e)})}})()
