(function(){
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],frames:[]};
  function pushErr(kind,msg){try{P[kind].push(String(msg).slice(0,300))}catch(e){}}
  window.addEventListener('error',function(e){pushErr('pageErrors',e.message||e.error)});
  window.addEventListener('unhandledrejection',function(e){pushErr('pageErrors',(e.reason&&e.reason.message)||e.reason)});
  var ce=console.error; console.error=function(){try{pushErr('consoleErrors',Array.prototype.map.call(arguments,String).join(' '))}catch(e){} return ce.apply(console,arguments)};
  var os=window.uni&&window.uni.request;
  if(os){window.uni.request=function(opt){try{var u=(opt&&opt.url)||'';if(u.indexOf('/battle/settle')>=0){var cs=opt.success;opt.success=function(res){try{P.frames.push({url:u,status:res&&res.statusCode,hash:res&&res.data&&res.data.replay_hash})}catch(e){} return cs&&cs(res)};var cf=opt.fail;opt.fail=function(res){pushErr('apiFailures',u+' '+(res&&res.statusCode));return cf&&cf(res)}}}catch(e){} return os.call(this,opt)};}
  window.__R24_PROBE__=P;
})();