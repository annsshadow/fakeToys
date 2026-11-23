(function(){
  try { localStorage.setItem('lyp_token', '__LYP_TOKEN__'); localStorage.removeItem('lyp_token_exp'); } catch(e) {}
  var P={pageErrors:[],consoleErrors:[],apiFailures:[],failedRequests:[],settleResponses:[],battleDetailResponses:[],qaShimApplied:0}; window.__LYP_PROBE__=P;
  window.addEventListener('error',function(e){P.pageErrors.push(String(e.message||''))});
  window.addEventListener('unhandledrejection',function(e){var m=String((e.reason&&e.reason.message)||e.reason||''); if(/play\(\) failed|user didn't interact|Autoplay/i.test(m))return; P.pageErrors.push('unhandled:'+m)});
  var oe=console.error; console.error=function(){try{P.consoleErrors.push(Array.prototype.map.call(arguments,function(a){return String(a)}).join(' '))}catch(e){} oe.apply(console,arguments)};
  var iv=setInterval(function(){
    if(window.uni && window.uni.request && !window.uni.__r15Patched){
      var orig=window.uni.request;
      window.uni.request=function(opts){
        try {
          // R15 P0 修复后：H5 已上报 snake_case(enemy_uid)，此处**不再篡改**任何字段。
          // 任何字段错配都会真实触发后端 fail loud → settle 500，探针会如实记入 apiFailures。
          var url=String((opts&&opts.url)||'');
          var os=opts.success;
          opts=Object.assign({},opts,{success:function(res){
            try{var st=res&&res.statusCode||0; if(st>=400)P.apiFailures.push({url:url,status:st});
              if(/\/battle\/settle/.test(url)){var j=res&&res.data||{};P.settleResponses.push({status:st,battle_id:(j.battle_id||(j.data&&j.data.battle_id))||0})}
              if(/\/me\/battles\/\d+/.test(url)){var d=(res&&res.data&&res.data.data)||(res&&res.data)||{}; var tl2=d.reaction_timeline; var arr=Array.isArray(tl2)?tl2:(tl2&&Array.isArray(tl2.events)?tl2.events:[]); P.battleDetailResponses.push({status:st,timelineLen:arr.length,schema:(tl2&&tl2.schema_version)||'',truncated:!!(tl2&&tl2.truncated),total:(tl2&&tl2.total)||0,first:arr.length?arr[0]:null})}
            }catch(e){} return os&&os.apply(this,arguments)
          }});
        }catch(e){}
        return orig.apply(this,arguments)
      };
      window.uni.__r15Patched=true;
    }
    if(window.uni&&window.uni.__r15Patched)clearInterval(iv);
  },30);
  setTimeout(function(){clearInterval(iv)},8000);
})();
