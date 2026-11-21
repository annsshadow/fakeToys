(function(){
  var txt = document.body.innerText || '';
  var hasSkip = txt.indexOf('全部不要了') >= 0;
  var hasResult = txt.indexOf('通关') >= 0 || txt.indexOf('战绩未入账') >= 0 || txt.indexOf('防线失守') >= 0;
  var waveM = txt.match(/波次\s*(\d+)\s*\/\s*(\d+)/);
  var killM = txt.match(/击杀\s*(\d+)/);
  var hpM = txt.match(/防线\s*(\d+)\s*\/\s*(\d+)/);
  var clicked = false;
  if (hasSkip && !hasResult) {
    var all = document.querySelectorAll('uni-view,view,text,.btn');
    for (var i=0;i<all.length;i++){
      var e=all[i];
      if ((e.innerText||'').trim()==='全部不要了，直接下一波' && e.children.length===0){ e.click(); clicked=true; break; }
    }
  }
  // 提取结算结果细节
  var settleDetail = '';
  if (hasResult) {
    var lines = txt.split(/\n/).filter(function(l){return /通关|评级|金币|体力|战绩|下一关|返回/.test(l);}).slice(0,8);
    settleDetail = lines.join(' | ');
  }
  return JSON.stringify({wave:waveM?waveM[1]+'/'+waveM[2]:null, kills:killM?killM[1]:null, hp:hpM?hpM[1]+'/'+hpM[2]:null, hasSkip:hasSkip, hasResult:hasResult, clicked:clicked, settle:hasResult?settleDetail:''});
})()