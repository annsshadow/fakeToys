(function(){
  var vs=document.querySelectorAll('.nick-edit-row .btn'); var save=null;
  for(var i=0;i<vs.length;i++){ var t=vs[i].textContent.trim(); if(t.indexOf('保存')>=0||t.indexOf('提交中')>=0){ save=vs[i]; break; } }
  if(!save) return JSON.stringify({clicked:false});
  var label=save.textContent.trim(); save.click(); return JSON.stringify({clicked:true,label:label});
})()
