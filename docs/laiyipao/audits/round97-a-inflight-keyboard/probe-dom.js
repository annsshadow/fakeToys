(function(){
  var out={url:location.href,title:document.title};
  out.isLogin=/login/.test(location.hash||'');
  var btns=[].slice.call(document.querySelectorAll('.btn, [role=button], .r7-btn'));
  out.buttons=btns.slice(0,20).map(function(b){return {tag:b.tagName,cls:b.className,role:b.getAttribute('role'),tabindex:b.getAttribute('tabindex'),tabIndex:b.tabIndex,text:(b.textContent||'').trim().slice(0,40)}});
  var inp=document.querySelector('input[type=number]');
  out.numInput=inp?{tag:inp.tagName,ph:inp.getAttribute('placeholder')}:null;
  try{out.lsKeys=Object.keys(localStorage)}catch(e){out.lsKeys=['ERR']}
  return JSON.stringify(out);
})()
