(() => {
  const qa=s=>Array.from(document.querySelectorAll(s)); const st=el=>getComputedStyle(el);
  const txt=el=>(el?.textContent||'').trim();
  const rect=el=>{if(!el)return null;const r=el.getBoundingClientRect();return {x:Math.round(r.x),right:Math.round(r.right),w:Math.round(r.width)};};
  const out={hash:location.hash,innerW:innerWidth,docW:document.documentElement.scrollWidth};
  out.badges=qa('.ann-new').map(e=>({text:txt(e),cls:e.className,rect:rect(e),display:st(e).display,visibility:st(e).visibility}));
  out.date0=qa('.ann-date')[0]?{text:txt(qa('.ann-date')[0]),rect:rect(qa('.ann-date')[0])}:null;
  out.rightOver=qa('*').filter(e=>{const r=e.getBoundingClientRect();return r.width>0&&r.right>window.innerWidth+1;}).slice(0,10).map(e=>({cls:(e.className||'').toString().slice(0,40),tag:e.tagName,right:Math.round(e.getBoundingClientRect().right),t:txt(e).slice(0,40)}));
  return out;
})()