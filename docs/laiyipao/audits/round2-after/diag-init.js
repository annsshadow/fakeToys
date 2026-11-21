(function(){
  window.__DIAG__={flag:true,urls:[],transport:[]};
  var of=window.fetch;
  window.fetch=function(){try{window.__DIAG__.urls.push(String(arguments[0]));window.__DIAG__.transport.push('fetch');}catch(_){}return of.apply(this,arguments);};
  var oo=XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open=function(m,u){try{window.__DIAG__.urls.push(String(u));window.__DIAG__.transport.push('xhr');}catch(_){}return oo.apply(this,arguments);};
})();