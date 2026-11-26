(function(){
  try { localStorage.setItem('lyp_token', '__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e) {}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],detailResponses:[]}; window.__LYP_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/Autoplay/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r25Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try {
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              if(url.indexOf('/me/battles/')>=0){var j=res&&res.data||{}; var d=j.data||j; P.detailResponses.push({status:st,battle_id:d.id||d.battle_id||0,tlLen:(d.reaction_timeline&&d.reaction_timeline.events||[]).length,schema:d.reaction_timeline&&d.reaction_timeline.schema_version||'',reactionsUsed:d.reactions_used||{}})}
            }catch(e){} return os&&os.apply(this,arguments)
          }});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r25Patched=true;
    }
    if(window.uni&&window.uni.__r25Patched)clearInterval(iv);
  },30);
  setTimeout(function(){clearInterval(iv)},8000);
})();