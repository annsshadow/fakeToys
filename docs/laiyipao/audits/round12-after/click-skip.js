(function(){
 var all=[].slice.call(document.querySelectorAll('*'));
 var b=all.filter(function(e){var t=(e.innerText||e.textContent||'').trim();return t.indexOf('全部不要了')>=0&&t.length<30}).pop();
 if(b){b.click();return 'CLICKED:'+b.tagName}else{return 'NOT_FOUND'}
})();