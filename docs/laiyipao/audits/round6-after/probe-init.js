// t57 R6 复查探针：捕获 pageError/unhandledrejection/console.error/失败请求/recommendation API 响应
(function(){
  var P=window.__LYP_PROBE__={pageErrors:[],consoleErrors:[],failedRequests:[],apiFailures:[],recResponses:[]};
  window.addEventListener("error",function(e){P.pageErrors.push(String(e.message||e.error).slice(0,300))});
  window.addEventListener("unhandledrejection",function(e){P.pageErrors.push("UNHANDLED:"+String((e.reason&&e.reason.message)||e.reason).slice(0,300))});
  var ce=console.error;console.error=function(){try{P.consoleErrors.push([].slice.call(arguments).map(String).join(" ").slice(0,300))}catch(e){}ce.apply(console,arguments)};
  function isRec(u){return /\/(me|admin)\/recommendations/.test(u)}
  var of=window.fetch;window.fetch=function(){var u=(typeof arguments[0]==="string")?arguments[0]:(arguments[0]&&arguments[0].url)||"";var p=of.apply(this,arguments);if(isRec(u)){p.then(function(r){P.recResponses.push({url:u,status:r.status});try{r.clone().json().then(function(j){P.recResponses[P.recResponses.length-1].body={schema_version:j.schema_version,context:j.context,empty_reason:j.empty_reason&&j.empty_reason.code,insights:(j.insights||[]).length,model:j.model&&j.model.name,headline:j.insights&&j.insights[0]&&j.insights[0].headline}}).catch(function(){})}catch(e){}}).catch(function(){})}return p};
  var os=XMLHttpRequest.prototype.send,oo=XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open=function(m,u){this.__u=u;return oo.apply(this,arguments)};
  XMLHttpRequest.prototype.send=function(){var self=this;this.addEventListener("load",function(){var u=self.__u||"";if(isRec(u)){var st=self.status;var rec={url:u,status:st};try{var j=JSON.parse(self.responseText);rec.body={schema_version:j.schema_version,context:j.context,empty_reason:j.empty_reason&&j.empty_reason.code,insights:(j.insights||[]).length,model:j.model&&j.model.name,headline:j.insights&&j.insights[0]&&j.insights[0].headline}}catch(e){}P.recResponses.push(rec)}});return os.apply(this,arguments)};
  window.addEventListener("error",function(e){var t=e.target;if(t&&(t.tagName==="SCRIPT"||t.tagName==="IMG"||t.tagName==="LINK")){P.failedRequests.push((t.src||t.href||"").slice(0,200))}},true);
})();