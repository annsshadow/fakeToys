(async function(){
  try {
    var t = localStorage.getItem('lyp_token');
    var r = await fetch('https://' + location.hostname + '/laiyipao-api/api/v1/me/battles?limit=3', {headers:{'Authorization':'Bearer ' + t}});
    var j = await r.json();
    var arr = j.items || (Array.isArray(j)?j:[]);
    return JSON.stringify({status:r.status, count:arr.length,
      latest: arr.slice(0,2).map(function(b){return {id:b.id, level:b.level_id, win:b.win, score:b.score, replay_hash:(b.replay_hash||'').slice(0,12)}})});
  } catch(e) { return 'ERR '+e.message; }
})()