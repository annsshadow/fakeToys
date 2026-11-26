(() => {
  const q=(s,r)=> (r||document).querySelector(s);
  const qa=(s,r)=> Array.from((r||document).querySelectorAll(s));
  const txt=(el)=> el?(el.textContent||'').trim():'';
  const cards=qa('.hand-card').map((hc,i)=>({
    idx:i, name:txt(q('.hand-name',hc)), cardElement:txt(q('.tag',hc)),
    r22card:!!q('.r22-synergy-card',hc),
    line:txt(q('.r22-synergy-line',hc)),
    tags:qa('.r22-synergy-tag',hc).map(t=>t.getAttribute('data-r22-reaction')),
    tagTexts:qa('.r22-synergy-tag',hc).map(t=>txt(t)),
    match:txt(q('.r22-synergy-match',hc)),
    empty:txt(q('.r22-synergy-empty',hc)),
  }));
  const note=q('.r22-ai-note'); const nt=txt(note);
  const p=window.__R22_PROBE__||{pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],settleResponses:[],startResponses:[]};
  return JSON.stringify({
    url:location.href, title:document.title,
    handCount:cards.length, cards,
    note:{exists:!!note, role:note&&note.getAttribute('role'), text:nt,
      hasAI:/AI/.test(nt),
      hasBasis:nt.includes('依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生'),
      hasCTA:nt.includes('去装备/选它'),
      hasHonesty:nt.includes('辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')},
    hasUndefined:document.body.textContent.includes('undefined'), hasNaN:document.body.textContent.includes('NaN'),
    pe:p.pageErrors, ce:p.consoleErrors, af:p.apiFailures, fr:p.failedRequests,
    settle:p.settleResponses, start:p.startResponses
  });
})()