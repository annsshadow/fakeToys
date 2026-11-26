(function(){
  var ins=document.querySelectorAll('input'); var inp=null;
  for(var i=0;i<ins.length;i++){
    var p=ins[i].closest?ins[i].closest('uni-input'):null;
    if((p&&(p.className||'').indexOf('nick-inp')>=0)||(ins[i].className||'').indexOf('nick-inp')>=0){inp=ins[i];break;}
  }
  if(!inp){ for(var k=0;k<ins.length;k++){ if((ins[k].className||'').indexOf('uni-input-input')>=0){inp=ins[k];break;} } }
  var vs=document.querySelectorAll('.nick-edit-row .btn'); var saves=[];
  for(var i=0;i<vs.length;i++){ var t=vs[i].textContent.trim(); if(t.indexOf('保存')>=0||t.indexOf('提交中')>=0){ saves.push({text:t,cls:vs[i].className||''}); } }
  var body=document.body.innerText||'';
  var errs=[]; var dims=document.querySelectorAll('.dim,uni-text.dim');
  for(var j=0;j<dims.length;j++){ var dt=(dims[j].textContent||'').trim(); if(dt.indexOf('无法连接')>=0||dt.indexOf('修改失败')>=0||dt.indexOf('request:fail')>=0||dt.indexOf('err')>=0&&dt.indexOf('昵称')>=0){ errs.push(dt); } }
  var qa=window.__qaNet||{puts:[]};
  var de=document.documentElement;
  return JSON.stringify({
    editing:!!inp, inputDisabled:inp?!!inp.disabled:null, inputValue:inp?inp.value:null,
    saveBtns:saves, saveLabels:saves.map(function(s){return s.text;}),
    btnDisabledCount:saves.filter(function(s){return /btn-disabled/.test(s.cls);}).length,
    errTexts:errs, errVisible:body.indexOf('无法连接')>=0||body.indexOf('修改失败')>=0||body.indexOf('request:fail')>=0,
    putCount:qa.puts.length, putBodies:qa.puts.map(function(p){return p.body;}),
    innerW:window.innerWidth, docW:de.scrollWidth||de.clientWidth,
    hOverflow:(de.scrollWidth||de.clientWidth)>window.innerWidth, bodyLen:body.length, hash:location.hash
  });
})()
