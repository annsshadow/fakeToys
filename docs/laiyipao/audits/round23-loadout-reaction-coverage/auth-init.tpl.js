(function(){
  try { localStorage.setItem('lyp_token','__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e){}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],meResponses:[],configResponses:[]}; window.__R23_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/Autoplay|user didn t interact|play\(\) failed/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r23Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try{ var url=String((opts&&opts.url)||''); var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{ var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              var j=(res&&res.data&&res.data.data)||(res&&res.data)||{};
              if(/\/me(\?|$)/.test(url)) P.meResponses.push({status:st,loadout:j.loadout||null});
              if(/\/config/.test(url)) P.configResponses.push({status:st,skills:(j.skills||[]).length,composites:(j.composite_skills||[]).length});
            }catch(e){} return os&&os.apply(this,arguments)
          },fail:function(e){try{P.failedRequests.push({url:String((opts&&opts.url)||''),err:String(e&&e.errMsg||e)})}catch(x){} var of=opts&&opts.fail;return of&&of.apply(this,arguments)}});
        }catch(e){} return orig.apply(this,arguments)
      }; window.uni.__r23Patched=true;
    }
    if(window.uni&&window.uni.__r23Patched)clearInterval(iv);
  },30); setTimeout(function(){clearInterval(iv)},10000);
})();
