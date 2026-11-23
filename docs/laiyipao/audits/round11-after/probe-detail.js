(function(){
  var pr=window.__LYP_PROBE__||{};
  function pathOf(u){try{return new URL(u,location.href).pathname}catch(e){return String(u).slice(0,40)}}
  var af=(pr.apiFailures||[]).map(function(x){return pathOf(x.url)+':'+x.status}).slice(0,8);
  var fr=(pr.failedRequests||[]).map(function(x){return pathOf(x.url)+':'+String(x.msg).slice(0,30)}).slice(0,8);
  var tk='';try{tk=localStorage.getItem('lyp_token')||''}catch(e){}
  var txt=(document.body.innerText||'');
  return JSON.stringify({vp:window.innerWidth+'x'+window.innerHeight,tokenLen:tk.length,af:af,fr:fr,
    hasCurrent:txt.indexOf('当前')>=0,hasDash:txt.indexOf('—')>=0,longTail:(txt.match(/[0-9]+\.[0-9]{6,}/g)||[]).slice(0,3),
    evVals:[].slice.call(document.querySelectorAll('.r6-ev-val')).map(function(e){return (e.innerText||'').trim()}).slice(0,5)});
})();