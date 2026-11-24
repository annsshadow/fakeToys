(() => {
  const q=(s,r)=> (r||document).querySelector(s);
  const qa=(s,r)=> Array.from((r||document).querySelectorAll(s));
  const txt=(el)=> el?(el.textContent||'').trim():'';
  const card=q('.r23-coverage-card');
  const tags=qa('.r23-coverage-tag',card).map(t=>({key:t.getAttribute('data-r23-reaction'),text:txt(t)}));
  const gap=txt(q('.r23-gap-suggest',card));
  const empty=txt(q('.r23-coverage-empty',card));
  const note=q('.r23-ai-note',card); const nt=txt(note);
  const p=window.__R23_PROBE__||{pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],meResponses:[],configResponses:[]};
  const body=document.body.textContent||'';
  return JSON.stringify({
    url:location.href, title:document.title, loginPage:/登录|login/i.test(body.slice(0,200)),
    cardExists:!!card, status:q('.r23-coverage-empty',card)?'empty':'ok',
    tags, tagsCount:tags.length, gap, empty,
    note:{exists:!!note, text:nt, hasAI:/AI/.test(nt),
      hasBasis:nt.includes('依据：构筑槽元素 × REACTION_PAIRS(源自引擎 lookupReaction)派生'),
      hasCTA:nt.includes('去调整出战配置'),
      hasHonesty:nt.includes('辅助诊断非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')},
    hasUndefined:body.includes('undefined'), hasNaN:body.includes('NaN'),
    pe:p.pageErrors, ce:p.consoleErrors, af:p.apiFailures, fr:p.failedRequests,
    me:p.meResponses, config:p.configResponses
  });
})()
