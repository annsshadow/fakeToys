(function(){
 var rows=[].slice.call(document.querySelectorAll('[class*=row],[class*=item],[class*=card]')).filter(function(e){var t=(e.innerText||'');return t.indexOf('失败')>=0||t.indexOf('第40关')>=0||t.indexOf('40 关')>=0});
 var out=rows.slice(0,8).map(function(e){return {cls:e.className,txt:(e.innerText||'').trim().slice(0,80)}});
 var body=(document.body.innerText||'');
 return JSON.stringify({url:location.href,title:document.title,len:body.length,hasFail:body.indexOf('失败')>=0,hasLose:body.indexOf('失守')>=0,rows:out});
})();