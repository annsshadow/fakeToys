(function(){
  var q=function(s){return document.querySelector(s)};
  var qa=function(s){return [].slice.call(document.querySelectorAll(s))};
  var rule=q('.star-rule-card');
  var ms=qa('.diff-milestone');
  var legend=qa('.diff-legend-item').map(function(e){return (e.innerText||'').trim()});
  var shapes=qa('.diff-shape').map(function(e){return (e.innerText||'').trim()});
  var threats=qa('.threat-tag').map(function(e){return (e.innerText||'').trim()});
  var p=window.__LYP_PROBE__||{};
  return JSON.stringify({
    url:location.href,title:document.title,bodyLen:(document.body.innerText||'').length,
    starRule:rule?(rule.innerText||'').trim().slice(0,500):null,
    milestoneCount:ms.length,firstMilestone:ms.length?(ms[0].innerText||'').trim().slice(0,140):null,
    legend:legend,shapes:shapes.filter(function(v,i,a){return a.indexOf(v)===i}),threats:threats.slice(0,24),
    pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length
  });
})()