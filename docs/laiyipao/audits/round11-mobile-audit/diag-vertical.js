(function(){
  var target=/当前不可达|专吃有元素层数的目标/;
  var els=document.querySelectorAll('uni-text,text,div,span,p');
  var found=[];
  for(var i=0;i<els.length;i++){var e=els[i];var t=(e.innerText||e.textContent||'').trim();
    if(target.test(t)&&t.length<40){
      var r=e.getBoundingClientRect();var cs=getComputedStyle(e);
      var par=e.parentElement;var pr=par?par.getBoundingClientRect():null;var pcs=par?getComputedStyle(par):null;
      found.push({t:t,w:Math.round(r.width),h:Math.round(r.height),writing:cs.writingMode,
        parentW:pr?Math.round(pr.width):0,parentDisplay:pcs?pcs.display:'',parentFlex:pcs?pcs.flexDirection:'',
        cls:String(e.className||'').slice(0,40),parentCls:par?String(par.className||'').slice(0,40):''});
      if(found.length>=3)break;
    }
  }
  if(found.length){
    var f=document.querySelectorAll('uni-text,text,div,span,p');
    for(var k=0;k<f.length;k++){var tt=(f[k].innerText||f[k].textContent||'').trim();if(target.test(tt)&&tt.length<40){f[k].scrollIntoView({block:'center'});break}}
  }
  return JSON.stringify(found)
})();