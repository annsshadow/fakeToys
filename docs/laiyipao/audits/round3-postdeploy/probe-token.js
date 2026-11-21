(async function(){
  try {
    var t = localStorage.getItem('lyp_token');
    var r = await fetch('https://' + location.hostname + '/laiyipao-api/api/v1/battle/token', {
      method:'POST',
      headers:{'Content-Type':'application/json','Authorization':'Bearer ' + t},
      body:JSON.stringify({level_id:1})
    });
    var j = await r.json();
    return JSON.stringify({status:r.status, token_id:j.token_id, seed:j.seed,
      level_id:j.level && j.level.id, wave:j.level && j.level.wave_count,
      build_skills:(j.build && j.build.skills || []).length, build_attacker:!!(j.build && j.build.attacker)});
  } catch(e) { return 'ERR ' + e.message; }
})()