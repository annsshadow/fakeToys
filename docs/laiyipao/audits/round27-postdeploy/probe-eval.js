(() => {
  const prog = document.querySelector('.r27-wave-progress');
  const progText = document.querySelector('.r27-progress-text');
  const progEmpty = document.querySelector('.r27-progress-empty');
  const fill = document.querySelector('.r27-progress-fill');
  const aiNote = document.querySelector('.r27-ai-note');
  const aiTitle = document.querySelector('.r27-ai-title');
  const aiConcl = document.querySelector('.r27-ai-conclusion');
  const aiBasis = document.querySelector('.r27-ai-basis');
  const aiCta = document.querySelector('.r27-ai-cta');
  const aiHonesty = document.querySelector('.r27-ai-honesty');
  const waveText = document.querySelector('.wave-text');
  const killText = document.querySelector('.kill-text');
  const starGap = document.querySelector('.star-gap-text');
  const timeBudget = document.querySelector('.time-budget-text');
  const r24 = document.querySelector('.r24-element-hud');
  const r24Rows = document.querySelectorAll('.r24-enemy-row').length;
  const r26 = document.querySelector('.r26-wave-brief');
  const cards = document.querySelectorAll('.hand-card');
  const skip = [...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  const rr = document.querySelector('.result-title');
  const hudRow = document.querySelector('.hud-row');
  const waveIdx = prog ? prog.getAttribute('data-r27-progress') : '';
  const txt = progText ? progText.textContent.trim().replace(/\s+/g,' ') : '';
  const m = txt.match(/本波\s*(\d+)\/(\d+)\s*·\s*([\d.]+)%/);
  const concl = aiConcl ? aiConcl.textContent.trim() : '';
  const cm = concl.match(/本波共\s*(\d+)\s*只，已处理\s*(\d+)\s*只，场上\s*(\d+)\s*只、待出场\s*(\d+)\s*只，清理进度\s*([\d.]+)%/);
  const fillW = fill ? (fill.style.width || '') : '';
  const docOverflow = document.documentElement.scrollWidth > document.documentElement.clientWidth + 1;
  const hudWrap = hudRow ? getComputedStyle(hudRow).flexWrap : '';
  const state = rr ? 'SETTLED' : ((cards.length && r26) ? 'CARD_PHASE' : (skip ? 'SKIP_ONLY' : 'WAIT'));
  const s = {
    state, hasProg: !!prog, waveIdx, progText: txt,
    domResolved: m ? Number(m[1]) : null, domTotal: m ? Number(m[2]) : null, domPct: m ? m[3] : null,
    emptyText: progEmpty ? progEmpty.textContent.trim() : '', fillW,
    hasAiNote: !!aiNote, aiTitle: aiTitle ? aiTitle.textContent.trim() : '', aiConcl: concl,
    aiNums: cm ? {total:Number(cm[1]),resolved:Number(cm[2]),onField:Number(cm[3]),pending:Number(cm[4]),pct:cm[5]} : null,
    aiBasis: aiBasis ? aiBasis.textContent.trim() : '', aiCta: aiCta ? aiCta.textContent.trim() : '', aiHonesty: aiHonesty ? aiHonesty.textContent.trim() : '',
    waveText: waveText ? waveText.textContent.trim() : '', killText: killText ? killText.textContent.trim() : '',
    starText: starGap ? starGap.textContent.trim() : '', budgetText: timeBudget ? timeBudget.textContent.trim() : '',
    hasR24: !!r24, r24Rows, hasR26: !!r26, cardCount: cards.length, hasSkip: !!skip,
    docOverflow, hudWrap, hasUndefined: (document.body.innerText||'').includes('undefined'), hasNaN: (document.body.innerText||'').includes('NaN'),
    result: rr ? rr.textContent.trim() : ''
  };
  return JSON.stringify(s);
})()