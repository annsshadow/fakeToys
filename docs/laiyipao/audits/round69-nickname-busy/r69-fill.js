(function(){
  var ins=document.querySelectorAll('input'); var inp=null;
  for(var i=0;i<ins.length;i++){
    var p=ins[i].closest?ins[i].closest('uni-input'):null;
    if((p&&(p.className||'').indexOf('nick-inp')>=0)||(ins[i].className||'').indexOf('nick-inp')>=0){inp=ins[i];break;}
  }
  if(!inp){ for(var k=0;k<ins.length;k++){ if((ins[k].className||'').indexOf('uni-input-input')>=0){inp=ins[k];break;} } }
  if(!inp) return JSON.stringify({ok:false,n:ins.length});
  var v='QAprobeR69';
  var setter=Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype,'value').set;
  setter.call(inp,v); inp.dispatchEvent(new Event('input',{bubbles:true}));
  return JSON.stringify({ok:true,cls:inp.className,value:inp.value});
})()
