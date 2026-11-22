(function(){
  var p=window.__LYP_PROBE__;
  var body=document.body.innerText||'';
  var q=function(s){return document.querySelector(s)};
  var qa=function(s){return document.querySelectorAll(s)};
  var out={
    url:location.href,
    title:document.title,
    hasToken:!!localStorage.getItem('lyp_token'),
    firstRunGuide:!!q('.first-run-guide'),
    guideText:q('.first-run-guide')?q('.first-run-guide').innerText.slice(0,180):'',
    persona:body.indexOf('新玩家先打第')>=0,
    starGap:q('.star-gap-text')?q('.star-gap-text').innerText:'',
    attribution:q('.settle-attribution')?q('.settle-attribution').innerText:'',
    impactCards:qa('.r7-impact').length,
    impactTitle:q('.r7-impact-title')?q('.r7-impact-title').innerText:'',
    impactEmpty:q('.r7-empty-note')?q('.r7-empty-note').innerText:'',
    hasFakeFill:body.indexOf('不展示虚构数值')>=0,
    hasOldClaim:body.indexOf('反应伤害与面板攻击力无关')>=0 || body.indexOf('关系不大')>=0,
    honestClaim:body.indexOf('面板攻击力也有贡献')>=0,
    recLabels:body.indexOf('规则引擎')>=0,
    localTag:body.indexOf('本地规则引擎·当前页面数据')>=0,
    apiTag:q('.api-tag')?q('.api-tag').innerText:'',
    probe:p?{pe:p.pageErrors.length,ce:p.consoleErrors.length,af:p.apiFailures.length,fr:p.failedRequests.length,rec:p.recResponses.length,impact:p.recResponses.filter(function(r){return r.url.indexOf('/impact')>=0}).length}:null,
    recDetails:p?p.recResponses.filter(function(r){return r.url.indexOf('recommendations')>=0}).slice(-8):[]
  };
  return JSON.stringify(out);
})()