(function(){
  if (window.__LYP_PROBE__) return;
  var p={pageErrors:[],consoleErrors:[],failedRequests:[],apiFailures:[],startedAt:Date.now()};
  window.__LYP_PROBE__=p;
  window.addEventListener('error',function(e){try{p.pageErrors.push({msg:e.message,ts:Date.now()})}catch(_){}});
  window.addEventListener('unhandledrejection',function(e){try{p.pageErrors.push({msg:'unhandledrejection: '+(e.reason&&e.reason.message?e.reason.message:String(e.reason)),ts:Date.now()})}catch(_){}});
  var oe=console.error;console.error=function(){try{p.consoleErrors.push([].map.call(arguments,function(a){return(a&&a.message)?a.message:String(a)}).join(' '))}catch(_){}return oe.apply(console,arguments)};
  var of=window.fetch;if(of){window.fetch=function(){var u=arguments[0];u=(typeof u==='string')?u:(u&&u.url)?u.url:'';return of.apply(this,arguments).then(function(r){try{if(r.status>=400)p.apiFailures.push({url:u,status:r.status,ts:Date.now()})}catch(_){}return r}).catch(function(e){try{p.failedRequests.push({url:u,err:String(e),ts:Date.now()})}catch(_){}throw e})}};
  var oo=XMLHttpRequest.prototype.open,os=XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.open=function(m,u){this.__u=u;return oo.apply(this,arguments)};
  XMLHttpRequest.prototype.send=function(){var s=this;this.addEventListener('load',function(){try{if(s.status>=400)p.apiFailures.push({url:s.__u,status:s.status,ts:Date.now()})}catch(_){}});this.addEventListener('error',function(){try{p.failedRequests.push({url:s.__u,err:'network error',ts:Date.now()})}catch(_){}});return os.apply(this,arguments)};
})();