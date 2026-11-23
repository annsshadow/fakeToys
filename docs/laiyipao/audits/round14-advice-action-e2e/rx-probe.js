(() => {
  const q=(s)=>document.querySelectorAll(s).length;
  const cta=document.querySelector('#r14-advice-action-btn.r14-advice-cta[data-r14-source="reactions"]');
  const badge=document.querySelector('.r14-advice-badge');
  const txt=document.querySelector('.r14-advice-text');
  const empty=document.querySelector('.r14-advice-empty[data-r14-source="reactions"]');
  const p=window.__LYP_PROBE__||{};
  const bodyText=document.body.innerText||'';
  const domainInBody=/https?:\/\//.test(bodyText);
  return JSON.stringify({url:location.hash,title:document.title,
    r14Advice:q('.r14-advice'),r14Empty:q('.r14-advice-empty'),r14Cta:q('#r14-advice-action-btn.r14-advice-cta[data-r14-source="reactions"]'),
    badge:badge?badge.textContent.trim():'',advText:txt?txt.textContent.trim().slice(0,140):'',
    emptyMsg:empty?empty.textContent.trim().slice(0,100):'',source:cta?cta.getAttribute('data-r14-source'):'',
    advResponses:(p.adviceResponses||[]).slice(0,8),
    pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,
    domainInBody:domainInBody});
})()