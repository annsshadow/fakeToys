(() => {
  const qa=s=>Array.from(document.querySelectorAll(s)); const st=el=>getComputedStyle(el);
  const t=el=>(el?.textContent||'').trim();
  const out={href:location.href,hash:location.hash,innerW:window.innerWidth,docW:document.documentElement.scrollWidth};
  out.r51radar=qa('.r51-radar').map(e=>({viewBox:e.getAttribute('viewBox'),rings:qa('.r51-radar-ring').length,axes:qa('.r51-radar-axis').length}));
  out.r51honesty=qa('.r51-radar-honesty').map(e=>t(e));
  out.r53concl=qa('.r3-ai-conclusion,.r3-ai-cta,[class*=ai-conclusion]').map(e=>t(e).slice(0,60));
  out.hasLongFloat=/\d{2}\.\d{10,}/.test(document.body.innerText);
  out.r14cta=qa('.r14-advice-empty-cta').map(e=>({t:t(e),ws:st(e).whiteSpace}));
  out.phColors=qa('.uni-input-placeholder,.ph').map(e=>st(e).color).filter((v,i,a)=>a.indexOf(v)===i);
  out.r12=qa('.r12-priority-text,.r12-priority-cta').map(e=>t(e).slice(0,40));
  out.r48self=qa('[class*=cta]').map(e=>t(e)).filter(x=>/在本页强化/.test(x));
  return out;
})()