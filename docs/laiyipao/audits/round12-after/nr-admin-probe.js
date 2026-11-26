(function(){var pr=window.__LYP_PROBE__||{};var t=(document.body.innerText||'');
 var api=document.querySelector('.api-tag');
 return JSON.stringify({url:location.href,title:document.title,isLogin:/login/.test(location.hash),
  apiTag:api?(api.innerText||'').trim():'',hasAiLabel:t.indexOf('AI')>=0,
  cmdTrigger:document.querySelectorAll('.command-trigger').length,
  pe:(pr.pageErrors||[]).length,ce:(pr.consoleErrors||[]).length,af:(pr.apiFailures||[]).length,fr:(pr.failedRequests||[]).length});
})();