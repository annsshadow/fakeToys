(async function(){
  var t=localStorage.getItem('lyp_token')||'';
  try{
    var r=await fetch('http://127.0.0.1:8080/api/v1/me/battles?limit=20',{headers:{Authorization:'Bearer '+t}});
    var txt=await r.text();
    if(txt.charAt(0)!=='{') return 'STATUS='+r.status+' NONJSON='+txt.slice(0,30);
    var j=JSON.parse(txt);var d=j.data||j;
    var arr=Array.isArray(d)?d:(d.items||d.list||d.records||[]);
    var hs=arr.slice(0,4).map(function(b){return (b.replay_hash||'').slice(0,16)+':'+b.score+':'+b.stars});
    return 'STATUS='+r.status+' COUNT='+arr.length+' HASHES='+hs.join('|');
  }catch(e){return 'ERR '+e.message}
})()