(function(){
  var bs=[].slice.call(document.querySelectorAll('button'));
  var b=bs.filter(function(x){return /登录/.test((x.innerText||''))})[0];
  if(b){b.click();return 'CLICKED_LOGIN'}
  return 'NO_LOGIN_BUTTON count='+bs.length;
})()