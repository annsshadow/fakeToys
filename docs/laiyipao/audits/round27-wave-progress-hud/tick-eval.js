(() => {
  const prog=document.querySelector('.r27-wave-progress');
  const pt=document.querySelector('.r27-progress-text');
  const txt=pt?pt.textContent.trim().replace(/\s+/g,' '):'';
  const m=txt.match(/本波\s*\d+\/(\d+)\s*·\s*([\d.]+)%/);
  const total=m?Number(m[1]):0;
  const pct=m?Number(m[2]):0;
  const idx=prog?prog.getAttribute('data-r27-progress'):'';
  const ptag=pct>=99.95?'FULL':(pct>=40?'MID':(pct>0?'LOW':'ZERO'));
  const cards=document.querySelectorAll('.hand-card').length;
  const skip=[...document.querySelectorAll('.btn')].some(b=>b.textContent.includes('全部不要了'))?1:0;
  const r26=!!document.querySelector('.r26-wave-brief');
  const rr=!!document.querySelector('.result-title');
  const state=rr?'SETTLED':((cards&&r26)?'CARD_PHASE':(skip?'SKIP_ONLY':'WAIT'));
  return [state,idx,total,ptag,cards,skip,pct.toFixed(1)].join('|');
})()