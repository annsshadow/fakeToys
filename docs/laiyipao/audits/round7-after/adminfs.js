(function(){
  var p=window.__LYP_PROBE__;
  var body=document.body.innerText||'';
  var q=function(s){return document.querySelector(s)};
  var out={
    url:location.href,
    title:document.title,
    apiTag:q('.api-tag')?q('.api-tag').innerText:'',
    domainInBody:false,
    cmd:q('.command-trigger')?1:0,
    probe:p?{pe:p.pageErrors.length,ce:p.consoleErrors.length,af:p.apiFailures.length,fr:p.failedRequests.length}:null,
    kicker:q('.section-kicker')?q('.section-kicker').innerText:'',
    ai:q('.ai-insight-card')?1:0
  };
  return JSON.stringify(out);
})()