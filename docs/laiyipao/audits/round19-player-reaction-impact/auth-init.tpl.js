(function(){
  try { localStorage.setItem('lyp_token','__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e){}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],impactResponses:[]}; window.__R19_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/play\(\) failed|user didn't interact|Autoplay/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r19Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try{
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              if(/reaction-damage-impact/.test(url)){
                var j=(res&&res.data&&res.data.data)||(res&&res.data)||{};
                P.impactResponses.push({url:url,status:st,window_days:j.window_days,window_from:j.window_from,window_to:j.window_to,excluded:j.excluded_old_battles,items:(j.items||[]).map(function(it){return {reaction:it.reaction,damage:it.total_damage,pct:it.damage_share_pct,events:it.events,count:it.count}})});
              }
            }catch(e){} return os&&os.apply(this,arguments)
          },fail:function(e){try{var url2=String((opts&&opts.url)||'');if(/reaction-damage-impact/.test(url2))P.failedRequests.push({url:url2,err:String(e&&e.errMsg||e)})}catch(x){} var of=opts&&opts.fail;return of&&of.apply(this,arguments)}});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r19Patched=true;
    }
    if(window.uni&&window.uni.__r19Patched)clearInterval(iv);
  },30);
  setTimeout(function(){clearInterval(iv)},10000);
})();
