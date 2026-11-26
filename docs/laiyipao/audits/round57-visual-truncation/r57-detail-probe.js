(() => {
  const qa=s=>Array.from(document.querySelectorAll(s)); const rect=el=>{if(!el)return null;const r=el.getBoundingClientRect();return {x:Math.round(r.x),right:Math.round(r.right),w:Math.round(r.width),y:Math.round(r.y)};};
  const st=el=>getComputedStyle(el); const txt=el=>(el?.textContent||'').trim();
  const out={href:location.href,innerW:window.innerWidth,docW:document.documentElement.scrollWidth};
  out.badges=qa('.ann-new').map(e=>({text:txt(e),cls:e.className,rect:rect(e),display:st(e).display,visibility:st(e).visibility,opacity:st(e).opacity}));
  const h=qa('.ann-head')[0];
  out.head=h?{rect:rect(h),children:Array.from(h.children).map(e=>({cls:(e.className||'').toString(),text:txt(e).slice(0,24),rect:rect(e),sw:e.scrollWidth,cw:e.clientWidth}))}:null;
  out.date0=qa('.ann-date')[0]?{text:txt(qa('.ann-date')[0]),rect:rect(qa('.ann-date')[0])}:null;
  out.phs=qa('.uni-input-placeholder,.ph,.input-placeholder').map(e=>({cls:(e.className||'').toString(),text:txt(e).slice(0,40),sw:e.scrollWidth,cw:e.clientWidth,rect:rect(e),overflow:st(e).overflow,textOverflow:st(e).textOverflow,whiteSpace:st(e).whiteSpace,minWidth:st(e).minWidth,color:st(e).color}));
  return out;
})()