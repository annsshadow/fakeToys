(function(){
 var all=[].slice.call(document.querySelectorAll('*'));
 var btns=all.filter(function(e){var t=(e.innerText||e.textContent||'').trim();return t.indexOf('全部不要了')>=0&&t.length<30});
 var txt=(document.body.innerText||'');
 return JSON.stringify({wave:(txt.match(/波次\s*\d+\s*\/\s*\d+/)||[])[0]||'',skipBtns:btns.length,
  skipText:btns.map(function(e){return (e.innerText||'').trim().slice(0,40)}),
  skipped:(txt.match(/跳过\s*\d+\s*波/)||[])[0]||'',settle:/结算|失败|胜利|通关/.test(txt.slice(-600))});
})();