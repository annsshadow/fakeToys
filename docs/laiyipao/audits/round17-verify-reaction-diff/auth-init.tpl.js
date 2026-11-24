(function(){
  try { localStorage.setItem('lyp_token', '__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e) {}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],replayResponses:[],verifyResponses:[],qaShimApplied:0}; window.__LYP_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/play\(\) failed|user didn't interact|Autoplay/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r17Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try{
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{
              var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              var j=(res&&res.data)||{}; var d=(j.data&&j.data!==undefined)?j.data:j;
              if(/\/replay/i.test(url)&&!/verify/.test(url)){
                var tl=d.reaction_timeline||d.data&&d.data.reaction_timeline||{};
                P.replayResponses.push({status:st,schema:tl.schema_version||'',events:(tl.events||[]).length,total:tl.total||0,truncated:!!tl.truncated});
              }
              if(/\/verify/.test(url)){P.verifyResponses.push({status:st,matched:!!(d.matched),body:JSON.stringify(d).slice(0,150)})}
            }catch(e){} return os&&os.apply(this,arguments)
          }});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r17Patched=true; clearInterval(iv)
    }
  },30);
  setTimeout(function(){clearInterval(iv)},8000);
})();