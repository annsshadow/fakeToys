(async function(){
  var t=localStorage.getItem('lyp_token')||'';
  var h={Authorization:'Bearer '+t};
  try{
    var r=await fetch('/api/v1/me/battles?limit=20',{headers:h});
    var j=await r.json();
    var d=j.data||j;
    var arr=Array.isArray(d)?d:(d.items||d.list||d.records||[]);
    var hs=arr.slice(0,4).map(function(b){return (b.replay_hash||'').slice(0,16)+':'+b.score+':'+b.stars});
    return 'STATUS='+r.status+' COUNT='+arr.length+' HASHES='+hs.join('|');
  }catch(e){return 'ERR '+e.message}
})()