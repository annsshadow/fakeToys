(() => {
  const lines = document.querySelectorAll('.reaction-line').length;
  const rows = document.querySelectorAll('.r29-timeline-row').length;
  const overlay = !!document.querySelector('.r29-timeline-overlay');
  const legend = !!document.querySelector('.legend-overlay');
  const cards = document.querySelectorAll('.hand-card').length;
  const skip = [...document.querySelectorAll('.btn')].some(b=>b.textContent.includes('全部不要了'))?1:0;
  const rr = !!document.querySelector('.result-title');
  const state = rr?'SETTLED':((cards&&document.querySelector('.r26-wave-brief'))?'CARD_PHASE':(skip?'SKIP_ONLY':'WAIT'));
  return [state,lines,rows,overlay?1:0,legend?1:0,cards,skip].join('|');
})()
