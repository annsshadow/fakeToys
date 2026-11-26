(async function(){
  try {
    var t = localStorage.getItem('lyp_token');
    var r = await fetch('https://' + location.hostname + '/laiyipao-api/api/v1/battles?limit=3', {
      headers:{'Authorization':'Bearer ' + t}
    });
    var j = await r.json();
    var arr = j.battles || j.items || j.records || (Array.isArray(j)?j:[]);
    return JSON.stringify({status:r.status, count:arr.length,
      latest: arr.slice(0,2).map(function(b){return {id:b.id, level:b.level_id, win:b.win, score:b.score, coin:b.coin_reward||b.loot_coin, hash:(b.replay_hash||'').slice(0,8)}}),
      probe: window.__LYP_PROBE__ ? {pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length} : null});
  } catch(e) { return 'ERR '+e.message; }
})()