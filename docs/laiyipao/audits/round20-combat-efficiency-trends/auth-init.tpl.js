(function(){
  try { localStorage.setItem('lyp_token','__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e){}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],efficiencyResponses:[]}; window.__R20_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/play\(\) failed|user didn't interact|Autoplay/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r20Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try{
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              if(/battle-efficiency/.test(url)){
                var j=(res&&res.data&&res.data.data)||(res&&res.data)||{};
                P.efficiencyResponses.push({url:url,status:st,window_days:j.window_days,window_from:j.window_from,window_to:j.window_to,battles:j.battles,shots:j.shots,hits:j.hits,accuracy_pct:j.accuracy_pct,avg_heat_max:j.avg_heat_max,leaked_per_battle:j.leaked_per_battle,avg_duration_ms:j.avg_duration_ms,buckets:(j.buckets||[]).map(function(b){return {bucket:b.bucket,battles:b.battles,shots:b.shots,hits:b.hits,accuracy_pct:b.accuracy_pct}}),ai:j.ai_insight});
              }
            }catch(e){} return os&&os.apply(this,arguments)
          },fail:function(e){try{var url2=String((opts&&opts.url)||'');if(/battle-efficiency/.test(url2))P.failedRequests.push({url:url2,err:String(e&&e.errMsg||e)})}catch(x){} var of=opts&&opts.fail;return of&&of.apply(this,arguments)}});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r20Patched=true;
    }
    if(window.uni&&window.uni.__r20Patched)clearInterval(iv);
  },30);
  setTimeout(function(){clearInterval(iv)},10000);
})();
