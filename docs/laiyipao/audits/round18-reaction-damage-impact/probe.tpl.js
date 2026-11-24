(function(){
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],impactResponses:[]};
  window.__R18_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||'error'))});
  window.addEventListener('unhandledrejection',function(e){P.pageErrors.push('unhandled:'+String(e.reason&&e.reason.message||e.reason))});
  var ce=console.error;console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,String).join(' '))}catch(e){}return ce.apply(console,arguments)};
  var of=window.fetch;window.fetch=function(){var a=arguments;return of.apply(this,a).then(function(r){try{var u=String((a[0]&&a[0].url)||a[0]||'');if(u.indexOf('reaction-damage-impact')>=0){var c=r.clone();c.json().then(function(j){P.impactResponses.push({status:r.status,body:j})}).catch(function(){})}}catch(e){}return r})};
  var os=XMLHttpRequest.prototype.open;XMLHttpRequest.prototype.open=function(m,u){this.__u=String(u);return os.apply(this,arguments)};
  var ss=XMLHttpRequest.prototype.send;XMLHttpRequest.prototype.send=function(){var self=this;this.addEventListener('load',function(){try{if(String(self.__u||'').indexOf('reaction-damage-impact')>=0){P.impactResponses.push({status:self.status,body:JSON.parse(self.responseText)})}}catch(e){}});return ss.apply(this,arguments)};
  try{localStorage.setItem('lyp_admin_token','__R18_TOKEN__')}catch(e){}
})();
