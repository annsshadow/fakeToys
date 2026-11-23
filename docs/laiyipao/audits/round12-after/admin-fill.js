(function(){
 var ins=[].slice.call(document.querySelectorAll('input'));
 var user=ins.filter(function(e){return e.type!=='password'}).pop();
 var pass=ins.filter(function(e){return e.type==='password'}).pop();
 if(!user||!pass)return 'NO_INPUTS:'+ins.length;
 var set=Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set;
 set.call(user,'admin');user.dispatchEvent(new Event('input',{bubbles:true}));
 set.call(pass,'__ADMIN_PASS__');pass.dispatchEvent(new Event('input',{bubbles:true}));
 return 'FILLED:'+user.value+':'+pass.value.length;
})();