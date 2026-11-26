(() => {
  var LONG='SUPERLONGUNBREAKABLETOKENXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789NEXT';
  var nm=document.querySelector('.reaction-name');
  var at=document.querySelector('.r28-attribution');
  window.__R30_STRESS__={name:nm?nm.textContent:'',attr:at?at.textContent:'',applied:false};
  if(nm)nm.textContent=LONG+nm.textContent;
  if(at)at.textContent=LONG+' 第1波 · #1';
  var ok=!!((nm&&nm.textContent.indexOf(LONG)>=0)&&(at&&at.textContent.indexOf(LONG)>=0));
  window.__R30_STRESS__.applied=ok;
  var feed=document.querySelector('.reaction-feed');
  var rows=[...document.querySelectorAll('.reaction-line')].map(function(r,i){
    var b=r.getBoundingClientRect();var a=r.querySelector('.r28-attribution');var n=r.querySelector('.reaction-name');
    var cn=n?getComputedStyle(n):null;var ca=a?getComputedStyle(a):null;
    return {i:i,right:Math.round(b.right),width:Math.round(b.width),sw:r.scrollWidth,cw:r.clientWidth,
      nameWS:cn?cn.whiteSpace:'',nameOW:cn?cn.overflowWrap:'',nameOverflow:cn?cn.overflow:'',nameText:n?n.textContent.slice(0,40):'',
      attrWS:ca?ca.whiteSpace:'',attrOW:ca?ca.overflowWrap:'',attrOverflow:ca?ca.overflow:'',attrText:a?a.textContent.slice(0,40):''};
  });
  return JSON.stringify({applied:ok,vw:window.innerWidth,feedSW:feed?feed.scrollWidth:0,feedCW:feed?feed.clientWidth:0,
    docOverflow:document.documentElement.scrollWidth>document.documentElement.clientWidth+1,
    anyOverflow:rows.some(function(r){return r.right>window.innerWidth;}),rows:rows});
})()
