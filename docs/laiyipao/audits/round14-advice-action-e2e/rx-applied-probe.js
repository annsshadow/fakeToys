(() => { const done=document.querySelector('.apply-done'); const err=document.querySelector('.r7-error');
  const p=window.__LYP_PROBE__||{};
  return JSON.stringify({done:done?done.textContent.trim():'',err:err?err.textContent.trim():'',
    pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});
})()