(() => {
  const q=(s,r=document)=>r.querySelector(s);
  const qa=(s,r=document)=>Array.from(r.querySelectorAll(s));
  const txt=(s)=>q(s)?.textContent?.trim()||'';
  const concl=txt('.ai-conclusion');
  let longFloat=false;
  for(let i=0;i<concl.length;i++){ if(concl[i]==='.'){ let j=i+1; while(j<concl.length && concl[j]>='0' && concl[j]<='9') j++; if(j-i>=4) longFloat=true; } }
  const meConcl=txt('.r3-ai-conclusion');
  let meLongFloat=false;
  for(let i=0;i<meConcl.length;i++){ if(meConcl[i]==='.'){ let j=i+1; while(j<meConcl.length && meConcl[j]>='0' && meConcl[j]<='9') j++; if(j-i>=4) meLongFloat=true; } }
  const radar=q('.r51-radar');
  return {
    href:location.href, hash:location.hash, innerW:innerWidth, docW:document.documentElement.scrollWidth,
    aiTitle:txt('.ai-title'), conclusion:concl, longFloat:longFloat,
    r12Text:txt('.r12-priority-text'), r12Cta:txt('.r12-priority-cta'),
    r3Title:txt('.r3-ai-title'), meConclusion:meConcl, meLongFloat:meLongFloat,
    meBasis:txt('.r3-ai-basis'),
    radar51:!!radar, radarViewBox:radar?.getAttribute('viewBox')||'',
    rings51:qa('.r51-radar-ring').length, axes51:qa('.r51-radar-axis').length,
    shape51:q('.r51-radar-shape')?.getAttribute('points')||'',
    honesty51:txt('.r51-radar-honesty'),
    bodyLen:document.body.innerText.length
  };
})()
