(() => {
  const q=(s,r=document)=>r.querySelector(s);
  const qa=(s,r=document)=>Array.from(r.querySelectorAll(s));
  const txt=(s)=>q(s)?.textContent?.trim()||'';
  const vis=(el)=>{if(!el)return false;const r=el.getBoundingClientRect(),st=getComputedStyle(el);return !!(r.width&&r.height&&st.display!=='none'&&st.visibility!=='hidden')};
  const concl=txt('.ai-conclusion');
  let longFloat=false;
  for(let i=0;i<concl.length;i++){ if(concl[i]==='.'){ let j=i+1; while(j<concl.length && concl[j]>='0' && concl[j]<='9') j++; if(j-i>=4) longFloat=true; } }
  const svg=q('.radar-svg');
  const sr=svg?.getBoundingClientRect();
  const dims=qa('.dim-row').map(row=>({
    label:q('.dim-label',row)?.textContent?.trim()||'',
    weight:q('.dim-weight',row)?.textContent?.trim()||'',
    width:q('.bar-fill',row)?.getAttribute('style')||'',
    value:q('.dim-value',row)?.textContent?.trim()||''
  }));
  const mg=qa('.mg-row').map(e=>e.textContent.trim());
  return {
    href:location.href, hash:location.hash, innerW:innerWidth, docW:document.documentElement.scrollWidth,
    aiTitle:txt('.ai-title'), aiBadge:txt('.ai-badge'), aiSub:txt('.ai-sub'),
    conclusion:concl, longFloat:longFloat,
    evidence:txt('.ai-evidence'), ctaText:txt('.ai-cta'),
    dims:dims, mg:mg,
    radar:!!svg, radarVisible:vis(svg), viewBox:svg?.getAttribute('viewBox')||'',
    rings:qa('.radar-ring').length, axes:qa('.radar-axis').length,
    shape:q('.radar-shape')?.getAttribute('points')||'',
    labels:qa('.radar-label').map(e=>e.textContent.trim()),
    radarRect:sr?{x:Math.round(sr.x),y:Math.round(sr.y),w:Math.round(sr.width),h:Math.round(sr.height)}:null,
    bodyLen:document.body.innerText.length
  };
})()
