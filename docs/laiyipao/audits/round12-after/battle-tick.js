(function(){
 var all=[].slice.call(document.querySelectorAll('*'));
 var b=all.filter(function(e){var t=(e.innerText||e.textContent||'').trim();return t.indexOf('全部不要了')>=0&&t.length<30}).pop();
 var txt=(document.body.innerText||'');
 var wave=(txt.match(/波次\s*\d+\s*\/\s*\d+/)||[])[0]||'';
 var settled=/防御失败|防线失守|结算|通关|下一关|失败/.test(txt.slice(0,900))&&/返回关卡|查看本局战报|再来一次/.test(txt);
 if(settled)return 'SETTLED:'+wave;
 if(b){b.click();return 'CLICKED:'+wave}
 return 'WAIT:'+wave;
})();