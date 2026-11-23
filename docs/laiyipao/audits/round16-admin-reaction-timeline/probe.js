(() => {
  const q=(s)=>document.querySelector(s), qa=(s)=>[].slice.call(document.querySelectorAll(s));
  const box=q('.r16-reaction-timeline');
  const rows=qa('.r16-timeline-row');
  const p=window.__LYP_PROBE__||{};
  const body=document.body.innerText||document.body.textContent||'';
  return JSON.stringify({
    url:location.href, title:document.title,
    loginPage:!!document.querySelector('input[placeholder="admin"]')||body.indexOf('管理员登录')>=0,
    drawerOpen:!!q('.el-drawer'),
    box:!!box, boxRole:box?box.getAttribute('role'):null,
    title:q('.r16-timeline-title')?q('.r16-timeline-title').textContent.trim():'',
    summary:q('.r16-timeline-summary')?q('.r16-timeline-summary').textContent.trim():'',
    basis:q('.r16-timeline-basis')?q('.r16-timeline-basis').textContent.trim():'',
    cta:q('.r16-timeline-cta')?q('.r16-timeline-cta').textContent.trim():'',
    empty:q('.r16-timeline-empty')?q('.r16-timeline-empty').textContent.trim():'',
    truncated:!!q('.r16-truncated-note'),
    rowsCount:rows.length,
    waves:rows.map(r=>r.getAttribute('data-r16-wave')),
    firstRow:rows[0]?rows[0].textContent.trim():'',
    lastRow:rows.length?rows[rows.length-1].textContent.trim():'',
    hasUndefined:box?box.textContent.indexOf('undefined')>=0:false,
    hasNaN:box?box.textContent.indexOf('NaN')>=0:false,
    hasR15:!!q('.r15-reaction-timeline'),
    pe:(p.pageErrors||[]).length, ce:(p.consoleErrors||[]).length,
    af:(p.apiFailures||[]).length, fr:(p.failedRequests||[]).length,
    detail:(p.detailResponses||[]).slice(-3),
    bodyHasDomain:/(?:https?:\/\/)[^\s]+|(?:admin12345|eyJ[A-Za-z0-9_-]{20,})/.test(body)
  });
})()