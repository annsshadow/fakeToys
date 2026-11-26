(function(){
  try { localStorage.setItem('lyp_token','__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e){}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],coachResponses:[]}; window.__R21_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/Autoplay|user didn't interact|play\(\) failed/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r21Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try{
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              if(/level-coach/.test(url)){
                var j=(res&&res.data&&res.data.data)||(res&&res.data)||{};
                P.coachResponses.push({url:url,status:st,window_days:j.window_days,window_from:j.window_from,window_to:j.window_to,items:(j.items||[]).map(function(it){return {level_id:it.level_id,attempts:it.attempts,wins:it.wins,best_wave:it.best_wave,avg_wave:it.avg_wave,last_result:it.last_result,stuck:it.stuck,coachTitle:it.coach&&it.coach.title}})});
              }
            }catch(e){} return os&&os.apply(this,arguments)
          },fail:function(e){try{var url2=String((opts&&opts.url)||'');if(/level-coach/.test(url2))P.failedRequests.push({url:url2,err:String(e&&e.errMsg||e)})}catch(x){} var of=opts&&opts.fail;return of&&of.apply(this,arguments)}});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r21Patched=true;
    }
    if(window.uni&&window.uni.__r21Patched)clearInterval(iv);
  },30);
  setTimeout(function(){clearInterval(iv)},10000);
})();
