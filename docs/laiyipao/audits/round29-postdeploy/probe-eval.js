(() => {
  const feed = document.querySelector('.reaction-feed');
  const lines = [...document.querySelectorAll('.reaction-line')].map(function(r){
    const a = r.querySelector('.r28-attribution');
    const nm = r.querySelector('.reaction-name');
    const dm = r.querySelector('.reaction-damage');
    const cu = r.querySelector('.reaction-cue');
    return { name: nm?nm.textContent.trim():'', damage: dm?dm.textContent.trim():'', attribution: a?a.textContent.trim():'', dataWave: a?a.getAttribute('data-r28-wave'):null, dataEnemy: a?a.getAttribute('data-r28-enemy'):null, cue: cu?cu.textContent.trim():'' };
  });
  const ov = document.querySelector('.r29-timeline-overlay');
  const rows = [...document.querySelectorAll('.r29-timeline-row')].map(function(r){
    return { idx: r.getAttribute('data-r29-index'), wave: r.getAttribute('data-r29-wave'), enemy: r.getAttribute('data-r29-enemy'), text: r.textContent.trim().replace(/\s+/g,' ').slice(0,120) };
  });
  const title = document.querySelector('.r29-timeline-head') || document.querySelector('.r29-timeline-overlay .head');
  const concl = document.querySelector('.r29-timeline-conclusion') || document.querySelector('.r29-timeline-ai') || document.querySelector('.r29-timeline-overlay .r29-ai-conclusion');
  const basis = document.querySelector('.r29-timeline-basis') || document.querySelector('.r29-timeline-overlay .r29-timeline-basis');
  const hon = document.querySelector('.r29-timeline-honesty');
  const empty = document.querySelector('.r29-timeline-empty');
  const trunc = document.querySelector('.r29-timeline-truncated');
  const cards = document.querySelectorAll('.hand-card');
  const skip = [...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  const rr = document.querySelector('.result-title');
  const legend = document.querySelector('.legend-overlay');
  const docOverflow = document.documentElement.scrollWidth > document.documentElement.clientWidth + 1;
  const feedOverflow = feed ? (feed.scrollWidth > feed.clientWidth + 1) : false;
  const state = rr?'SETTLED':((cards.length&&document.querySelector('.r26-wave-brief'))?'CARD_PHASE':(skip?'SKIP_ONLY':'WAIT'));
  const s = { state, hasFeed:!!feed, feedLines:lines, lineCount:lines.length,
    timelineOpen:!!ov, timelineRows:rows, rowCount:rows.length,
    timelineTitle:title?title.textContent.trim():'', conclusion:concl?concl.textContent.trim():'',
    basis:basis?basis.textContent.trim():'', honesty:hon?hon.textContent.trim():'',
    emptyText:empty?empty.textContent.trim():'', truncatedText:trunc?trunc.textContent.trim():'',
    hasR24:!!document.querySelector('.r24-element-hud'), hasR26:!!document.querySelector('.r26-wave-brief'), hasR27:!!document.querySelector('.r27-wave-progress'), hasR28:!!document.querySelector('.r28-feed-cta')||!!document.querySelector('.r28-feed-ai'),
    helpVisible:!!legend, cardCount:cards.length, hasSkip:!!skip,
    docOverflow, feedOverflow, hasUndefined:(document.body.innerText||'').includes('undefined'), hasNaN:(document.body.innerText||'').includes('NaN'),
    result: rr?rr.textContent.trim():'' };
  return JSON.stringify(s);
})()
