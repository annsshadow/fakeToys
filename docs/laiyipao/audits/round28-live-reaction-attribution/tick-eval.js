(() => {
  const lines = document.querySelectorAll('.reaction-line').length;
  const avail = document.querySelectorAll('.r28-attribution[data-r28-enemy]:not([data-r28-enemy="unknown"])').length;
  const ai = document.querySelector('.r28-feed-ai-conclusion');
  const aiTxt = ai ? ai.textContent.trim() : '';
  const dm = aiTxt.match(/已归因\s*(\d+)\/(\d+)/);
  const ratio = dm ? (dm[1]+'/'+dm[2]) : '-';
  const cards = document.querySelectorAll('.hand-card').length;
  const skip = [...document.querySelectorAll('.btn')].some(b=>b.textContent.includes('全部不要了'))?1:0;
  const rr = !!document.querySelector('.result-title');
  const legend = !!document.querySelector('.legend-overlay');
  const state = rr?'SETTLED':((cards&&document.querySelector('.r26-wave-brief'))?'CARD_PHASE':(skip?'SKIP_ONLY':'WAIT'));
  return [state,lines,avail,ratio,legend?1:0,cards,skip].join('|');
})()
