(() => {
  const card=document.querySelector('.r25-reaction-impact');
  const rows=[...document.querySelectorAll('.r25-impact-row')];
  const empty=document.querySelector('.r25-impact-empty');
  const concl=document.querySelector('.r25-ai-conclusion');
  const basis=document.querySelector('.r25-ai-basis');
  const cta=document.querySelector('.r25-ai-cta');
  const honesty=document.querySelector('.r25-ai-honesty');
  const note=document.querySelector('.r25-truncated-note');
  const tlRows=document.querySelectorAll('.r15-timeline-row').length;
  const p=window.__LYP_PROBE__||{};
  const login=!!document.querySelector('.login-page')||/登录|手机号|验证码/.test((document.body.innerText||'').slice(0,200))&&!card;
  return JSON.stringify({
    url:location.href,loginPage:login,card:!!card,role:card?card.getAttribute('role'):null,
    timelineRows:tlRows,
    rows:rows.map(function(r){return {reaction:r.getAttribute('data-r25-reaction'),text:r.textContent.replace(/\s+/g,' ').trim()}}),
    empty:empty?empty.textContent.trim():'',
    aiTitle:card&&card.querySelector('.r25-impact-label')?card.querySelector('.r25-impact-label').textContent.trim():'',
    conclusion:concl?concl.textContent.trim():'',
    basis:basis?basis.textContent.trim():'',
    cta:cta?cta.textContent.trim():'',
    honesty:honesty?honesty.textContent.trim():'',
    truncated:note?note.textContent.trim():'',
    hasUndefined:(document.body.innerText||'').includes('undefined'),hasNaN:(document.body.innerText||'').includes('NaN'),
    detail:(p.detailResponses||[]).slice(-1),
    pe:(p.pageErrors||[]).filter(function(x){return !/Autoplay|play\(\)/i.test(x)}).length,
    ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length
  });
})()