// 模板：__LYP_TOKEN__ 运行时注入，采集后删除生成文件，真值绝不入库
(function(){
  try{localStorage.setItem('lyp_token','__LYP_TOKEN__');localStorage.removeItem('lyp_token_exp')}catch(e){}
  if(window.__LYP_PROBE__)return;
  var p={pageErrors:[],consoleErrors:[],failedRequests:[],apiFailures:[],startedAt:Date.now()};window.__LYP_PROBE__=p;
  window.addEventListener('error',function(e){try{p.pageErrors.push({msg:e.message,ts:Date.now()})}catch(_){}});
  window.addEventListener('unhandledrejection',function(e){try{p.pageErrors.push({msg:'ur: '+(e.reason&&e.reason.message?e.reason.message:String(e.reason)),ts:Date.now()})}catch(_){}});
  var oe=console.error;console.error=function(){try{p.consoleErrors.push([].map.call(arguments,function(a){return(a&&a.message)?a.message:String(a)}).join(' '))}catch(_){}return oe.apply(console,arguments)};
})();