(() => {
  const q=(s,r=document)=>r.querySelector(s);
  const qa=(s,r=document)=>Array.from(r.querySelectorAll(s));
  const txt=(s)=>q(s)?.textContent?.trim()||'';
  const vis=(el)=>{if(!el)return false;const r=el.getBoundingClientRect(),st=getComputedStyle(el);return !!(r.width&&r.height&&st.display!=='none'&&st.visibility!=='hidden')};
  const svg=q('.r51-radar');
  const sr=svg?.getBoundingClientRect();
  const cta=q('.r3-ai-cta');
  const cr=cta?.getBoundingClientRect();
  return {
    href:location.href, hash:location.hash, innerW:innerWidth, docW:document.documentElement.scrollWidth,
    page:txt('.section-kicker-cn'),
    aiTitle:txt('.r3-ai-title'), aiBadge:txt('.r3-ai-badge'),
    conclusion:txt('.r3-ai-conclusion'), basis:txt('.r3-ai-basis'),
    weakText:qa('.r3-row-main').map(e=>e.textContent.trim()).join('|'),
    ctaText:txt('.r3-ai-cta'), ctaVisible:vis(cta), ctaCursor:cta?getComputedStyle(cta).cursor:null,
    ctaRect:cr?{x:Math.round(cr.x),y:Math.round(cr.y),w:Math.round(cr.width),h:Math.round(cr.height),cx:Math.round(cr.x+cr.width/2),cy:Math.round(cr.y+cr.height/2)}:null,
    radar:!!svg, radarVisible:vis(svg), radarViewBox:svg?.getAttribute('viewBox')||'', radarAria:svg?.getAttribute('aria-label')||'',
    radarRect:sr?{x:Math.round(sr.x),y:Math.round(sr.y),w:Math.round(sr.width),h:Math.round(sr.height)}:null,
    rings:qa('.r51-radar-ring').map(e=>e.getAttribute('points')),
    axes:qa('.r51-radar-axis').length, shape:q('.r51-radar-shape')?.getAttribute('points')||'',
    labels:qa('.r51-radar-label').map(e=>({text:e.textContent.trim(),x:+e.getAttribute('x'),y:+e.getAttribute('y')})),
    labelOverlap:qa('.r51-radar-label').some((a,i)=>qa('.r51-radar-label').slice(i+1).some(b=>Math.abs(+a.getAttribute('x')-+b.getAttribute('x'))<8&&Math.abs(+a.getAttribute('y')-+b.getAttribute('y'))<8)),
    honesty:txt('.r51-radar-honesty'), honestyVisible:vis(q('.r51-radar-honesty')),
    emptyTitle:txt('.r3-empty-title'), emptyReason:txt('.r3-empty-reason'),
    bodyLen:document.body.innerText.length
  };
})()
