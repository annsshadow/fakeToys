(() => {
  const q=(s,r=document)=>r.querySelector(s);
  const qa=(s,r=document)=>Array.from(r.querySelectorAll(s));
  const txt=(s)=>q(s)?.textContent?.trim()||'';
  const vis=(el)=>{if(!el)return false;const r=el.getBoundingClientRect(),st=getComputedStyle(el);return !!(r.width&&r.height&&st.display!=='none'&&st.visibility!=='hidden')};
  const cta=q('#ai-insight .ai-cta');
  const cr=cta?.getBoundingClientRect();
  return {
    href:location.href,hash:location.hash,innerW:innerWidth,docW:document.documentElement.scrollWidth,
    title:txt('#ai-insight .ai-title'),badge:txt('#ai-insight .ai-badge'),
    conclusion:txt('#ai-insight .ai-conclusion'),evidence:txt('#ai-insight .ai-evidence'),
    ctaText:txt('#ai-insight .ai-cta'),ctaVisible:vis(cta),
    ctaRect:cr?{x:Math.round(cr.x),y:Math.round(cr.y),w:Math.round(cr.width),h:Math.round(cr.height),cx:Math.round(cr.x+cr.width/2),cy:Math.round(cr.y+cr.height/2)}:null,
    dims:qa('.ai-dim').map(e=>e.textContent.trim()).slice(0,8),
    bodyLen:document.body.innerText.length
  };
})()
