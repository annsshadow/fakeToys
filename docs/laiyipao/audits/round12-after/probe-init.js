(function(){
  var p={pageErrors:[],consoleErrors:[],failedRequests:[],apiFailures:[],startedAt:Date.now()};window.__LYP_PROBE__=p;
  window.addEventListener('error',function(e){try{p.pageErrors.push({msg:e.message,ts:Date.now()})}catch(_){}});
  window.addEventListener('unhandledrejection',function(e){try{p.pageErrors.push({msg:'ur: '+(e.reason&&e.reason.message?e.reason.message:String(e.reason)),ts:Date.now()})}catch(_){}});
  var oe=console.error;console.error=function(){try{p.consoleErrors.push([].map.call(arguments,function(a){return(a&&a.message)?a.message:String(a)}).join(' '))}catch(_){}return oe.apply(console,arguments)};
  var of=window.fetch;window.fetch=function(){var u=arguments[0];var url=typeof u==='string'?u:(u&&u.url)||'';var a=arguments;return of.apply(this,a).then(function(r){try{if(r.status>=400)p.apiFailures.push({url:url,status:r.status})}catch(_){}return r}).catch(function(e){try{p.failedRequests.push({url:url,msg:String(e)})}catch(_){}throw e})};
  var os=XMLHttpRequest.prototype.send;XMLHttpRequest.prototype.send=function(){var x=this;try{x.addEventListener('loadend',function(){try{if(x.status>=400)p.apiFailures.push({url:x.responseURL||'',status:x.status})}catch(_){}})}catch(_){}return os.apply(this,arguments)};
})();
