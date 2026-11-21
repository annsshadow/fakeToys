(function(){
  if (window.__LYP_PROBE__) return;
  var p = { pageErrors: [], consoleErrors: [], failedRequests: [], apiFailures: [], startedAt: Date.now() };
  window.__LYP_PROBE__ = p;
  window.addEventListener('error', function(e){
    try { p.pageErrors.push({ msg: e.message, src: e.filename, line: e.lineno, ts: Date.now() }); } catch(_){}
  });
  window.addEventListener('unhandledrejection', function(e){
    try { p.pageErrors.push({ msg: 'unhandledrejection: ' + (e.reason && e.reason.message ? e.reason.message : String(e.reason)), ts: Date.now() }); } catch(_){}
  });
  var origErr = console.error;
  console.error = function(){
    try { p.consoleErrors.push(Array.prototype.map.call(arguments, function(a){ return (a && a.message) ? a.message : String(a); }).join(' ')); } catch(_){}
    return origErr.apply(console, arguments);
  };
  var origFetch = window.fetch;
  if (origFetch) {
    window.fetch = function(){
      var url = arguments[0];
      var u = (typeof url === 'string') ? url : (url && url.url) ? url.url : '';
      return origFetch.apply(this, arguments).then(function(res){
        try { if (res.status >= 400) p.apiFailures.push({ url: u, status: res.status, ts: Date.now() }); } catch(_){}
        return res;
      }).catch(function(err){
        try { p.failedRequests.push({ url: u, err: String(err), ts: Date.now() }); } catch(_){}
        throw err;
      });
    };
  }
  var origOpen = XMLHttpRequest.prototype.open;
  var origSend = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.open = function(m, u){ this.__lypUrl = u; return origOpen.apply(this, arguments); };
  XMLHttpRequest.prototype.send = function(){
    var self = this;
    this.addEventListener('load', function(){ try { if (self.status >= 400) p.apiFailures.push({ url: self.__lypUrl, status: self.status, ts: Date.now() }); } catch(_){} });
    this.addEventListener('error', function(){ try { p.failedRequests.push({ url: self.__lypUrl, err: 'network error', ts: Date.now() }); } catch(_){} });
    return origSend.apply(this, arguments);
  };
})();