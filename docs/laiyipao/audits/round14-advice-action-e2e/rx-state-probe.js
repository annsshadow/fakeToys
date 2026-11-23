(() => { const q=(s)=>document.querySelectorAll(s).length;
  const cta=document.querySelector('#r14-advice-action-btn');
  const empty=document.querySelector('.r14-advice-empty');
  const p=window.__LYP_PROBE__||{};
  return JSON.stringify({url:location.hash,title:document.title,
    r14Advice:q('.r14-advice'),r14Empty:q('.r14-advice-empty'),r14Cta:q('#r14-advice-action-btn'),
    emptyMsg:empty?empty.textContent.trim().slice(0,110):'',
    source:cta?(cta.getAttribute('data-r14-source')||''):'',
    advResponses:(p.adviceResponses||[]).slice(0,4),
    pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,
    domainInBody:/https?:\/\//.test(document.body.innerText||'')});
})()