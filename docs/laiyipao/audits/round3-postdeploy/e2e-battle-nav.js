(async function(){
  var out = {steps: []};
  function log(s){ out.steps.push(s); }
  function sleep(ms){ return new Promise(function(r){ setTimeout(r, ms); }); }
  function clickText(txt){
    var all = document.querySelectorAll('uni-view, view, text, button, .btn');
    for (var i=0;i<all.length;i++){
      var e=all[i];
      if ((e.innerText||'').trim()===txt && e.children.length===0){ e.click(); return true; }
    }
    return false;
  }
  try {
    log('url=' + location.href);
    // 进入战斗页（通过 hash 路由）
    location.hash = '#/pages/battle/battle';
    await sleep(4000);
    log('after_nav=' + location.href);
    log('body=' + document.body.innerText.slice(0,300).replace(/\n/g,' '));
    return JSON.stringify(out);
  } catch(e) { return 'ERR '+e.message+' steps='+JSON.stringify(out.steps); }
})()