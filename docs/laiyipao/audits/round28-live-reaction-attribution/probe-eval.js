(() => {
  const feed = document.querySelector('.reaction-feed');
  const lines = [...document.querySelectorAll('.reaction-line')].map(function(r){
    const a = r.querySelector('.r28-attribution');
    const nm = r.querySelector('.reaction-name');
    const dm = r.querySelector('.reaction-damage');
    const cu = r.querySelector('.reaction-cue');
    return {
      name: nm ? nm.textContent.trim() : '',
      damage: dm ? dm.textContent.trim() : '',
      attribution: a ? a.textContent.trim() : '',
      dataWave: a ? a.getAttribute('data-r28-wave') : null,
      dataEnemy: a ? a.getAttribute('data-r28-enemy') : null,
      cue: cu ? cu.textContent.trim() : ''
    };
  });
  const aiTitle = document.querySelector('.r28-feed-ai-title');
  const aiConcl = document.querySelector('.r28-feed-ai-conclusion');
  const aiBasis = document.querySelector('.r28-feed-ai-basis');
  const aiCta = document.querySelector('.r28-feed-cta');
  const aiHonesty = document.querySelector('.r28-feed-ai-honesty');
  const cards = document.querySelectorAll('.hand-card');
  const skip = [...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  const rr = document.querySelector('.result-title');
  const legend = document.querySelector('.legend-overlay');
  const docOverflow = document.documentElement.scrollWidth > document.documentElement.clientWidth + 1;
  const feedOverflow = feed ? (feed.scrollWidth > feed.clientWidth + 1) : false;
  const state = rr ? 'SETTLED' : ((cards.length && document.querySelector('.r26-wave-brief')) ? 'CARD_PHASE' : (skip ? 'SKIP_ONLY' : 'WAIT'));
  const s = {
    state, hasFeed: !!feed, feedLines: lines, lineCount: lines.length,
    aiTitle: aiTitle?aiTitle.textContent.trim():'', aiConcl: aiConcl?aiConcl.textContent.trim():'',
    aiBasis: aiBasis?aiBasis.textContent.trim():'', aiCta: aiCta?aiCta.textContent.trim():'',
    aiHonesty: aiHonesty?aiHonesty.textContent.trim():'',
    hasR24: !!document.querySelector('.r24-element-hud'), hasR26: !!document.querySelector('.r26-wave-brief'),
    hasR27: !!document.querySelector('.r27-wave-progress'),
    helpVisible: !!legend,
    cardCount: cards.length, hasSkip: !!skip,
    docOverflow, feedOverflow,
    hasUndefined: (document.body.innerText||'').includes('undefined'), hasNaN: (document.body.innerText||'').includes('NaN'),
    result: rr ? rr.textContent.trim() : ''
  };
  return JSON.stringify(s);
})()
