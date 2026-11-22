(function(){
  var q=function(s){return document.querySelector(s)};
  var qa=function(s){return [].slice.call(document.querySelectorAll(s))};
  var title=document.title||'';
  var kicker=q('.section-kicker');
  var kickerTxt=kicker?(kicker.innerText||'').trim():'';
  var aiBadges=qa('*').filter(function(e){return e.children.length===0&&(e.innerText||'').trim()==='AI'}).length;
  var body=(document.body.innerText||'');
  return JSON.stringify({
    hash:location.hash,pathname:location.pathname,title:title,bodyLen:body.length,
    hasWikiTitle:title.indexOf('玩家玩法文档站')>=0,
    hasDashboard:body.indexOf('核心指标')>=0||body.indexOf('运营概览')>=0||body.indexOf('AI 运营洞察')>=0,
    kicker: kickerTxt.slice(0,80),hasKickerCN:kickerTxt.indexOf('玩法知识库')>=0,hasKickerEN:kickerTxt.indexOf('PLAYER GUIDE')>=0,
    aiBadges:aiBadges,hasGuideDoc:body.indexOf('玩法')>=0&&(body.indexOf('关卡')>=0||body.indexOf('元素')>=0),
    isLogin: /login/i.test(location.hash),
    loginForm: qa('input').length>0&&/登录/.test(body)
  });
})()