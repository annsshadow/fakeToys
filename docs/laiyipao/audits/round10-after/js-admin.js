(function(){
  var p=window.__LYP_PROBE__||{};var b=document.body,t=b.innerText||'';
  function q(s){return document.querySelectorAll(s).length}
  var tag=document.querySelector('.api-tag');
  return JSON.stringify({url:location.href,title:document.title,
    isLogin:/\/login|登录页/.test(location.href)&&q('el-form')>0,
    apiTag:tag?(tag.innerText||'').trim():'',
    kicker:q('.section-kicker'),ai:q('.ai-insight-card'),
    cmdTrigger:q('.command-trigger'),textLen:t.length,
    probe:{pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}
  })
})()