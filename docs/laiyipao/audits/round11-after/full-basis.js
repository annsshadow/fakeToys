(function(){
  var out=[];
  ['.r6-ev-val','.r7-basis'].forEach(function(sel){
    [].slice.call(document.querySelectorAll(sel)).forEach(function(e){out.push({sel:sel,txt:(e.innerText||e.textContent||'').trim()})})
  });
  return JSON.stringify(out.slice(0,12));
})();
