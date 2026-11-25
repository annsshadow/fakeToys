
(() => {
  const q=(s,r=document)=>r.querySelector(s);
  const qa=(s,r=document)=>Array.from(r.querySelectorAll(s));
  const txt=(s)=>q(s)?.textContent?.trim()||'';
  const cta=q('.r12-priority-cta');
  const rect=cta?.getBoundingClientRect();
  const vis=(el)=>{if(!el)return false; const r=el.getBoundingClientRect(),st=getComputedStyle(el);return !!(r.width&&r.height&&st.display!=='none'&&st.visibility!=='hidden')};
  const tabDefs=qa('.bag-tabs .chip').map(e=>({text:e.textContent.trim(),active:e.classList.contains('active')||e.classList.contains('hit')}));
  const gearChips=qa('.r3-filter .r3-chip').map(e=>({text:e.textContent.trim(),active:e.classList.contains('active')}));
  return {
    href:location.href, hash:location.hash, innerW:innerWidth, docW:document.documentElement.scrollWidth,
    ctaText:txt('.r12-priority-cta'), ctaVisible:vis(cta), ctaCursor:cta?getComputedStyle(cta).cursor:null,
    ctaRect:rect?{x:Math.round(rect.x),y:Math.round(rect.y),w:Math.round(rect.width),h:Math.round(rect.height),cx:Math.round(rect.x+rect.width/2),cy:Math.round(rect.y+rect.height/2)}:null,
    priorityText:txt('.r12-priority-text'), tabDefs, gearChips,
    skillsCardVisible:vis(q('.bag-tabs ~ .card')),
    skillLibraryVisible:qa('.card-title').some(e=>e.textContent.includes('技能库')),
    moreCardVisible:qa('.card-title').some(e=>e.textContent.includes('皮肤')),
    bodyLen:document.body.innerText.length
  };
})()
