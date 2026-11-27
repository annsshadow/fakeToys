(()=>{
 var items=[].slice.call(document.querySelectorAll('.grid-item'));
 var me=items.filter(function(x){return (x.innerText||'').trim()==='我的'})[0];
 if(!me) return JSON.stringify({clicked:false,reason:'no-me-item',count:items.length});
 me.click();
 return JSON.stringify({clicked:true,label:(me.innerText||'').trim()});
})()
