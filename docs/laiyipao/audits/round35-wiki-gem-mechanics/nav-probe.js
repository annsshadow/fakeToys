(() => { try {
  const qa=(s)=>Array.from(document.querySelectorAll(s)); const txt=e=>e?(e.innerText||e.textContent||'').trim():"";
  const navs=qa(".nav-item").map(a=>({label:txt(a),href:a.getAttribute("href")}));
  const secs=qa("section.wiki-section").map(s=>({title:txt(s.querySelector("h2,h3,.section-title"))||txt(s).slice(0,20),visible:s.offsetParent!==null,cls:s.className}));
  const r35sec=qa(".r35-gem-mechanics");
  return JSON.stringify({navCount:navs.length,navs:navs,sectionCount:secs.length,gemSectionCount:r35sec.length,gemSectionVisible:!!r35sec[0]&&r35sec[0].offsetParent!==null,hash:location.hash});
}catch(e){return JSON.stringify({error:String(e)})}})()
