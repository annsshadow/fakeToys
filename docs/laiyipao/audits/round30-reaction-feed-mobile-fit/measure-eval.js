(() => {
  const vw = window.innerWidth;
  const feed = document.querySelector('.reaction-feed');
  const fr = feed?feed.getBoundingClientRect():null;
  const rows = [...document.querySelectorAll('.reaction-line')].map(function(r,i){
    const b=r.getBoundingClientRect();
    const a=r.querySelector('.r28-attribution');
    const nm=r.querySelector('.reaction-name');
    const ac=a?getComputedStyle(a):null;
    const ncs=nm?getComputedStyle(nm):null;
    return {i:i,right:Math.round(b.right),width:Math.round(b.width),sw:r.scrollWidth,cw:r.clientWidth,
      ws:ac?ac.whiteSpace:'', ow:ac?ac.overflowWrap:'', maxW:ac?ac.maxWidth:'', ov:ac?ac.overflow:'',
      attrText:a?a.textContent.trim():'', attrRight:a?Math.round(a.getBoundingClientRect().right):null,
      attrSW:a?a.scrollWidth:null, attrCW:a?a.clientWidth:null,
      nameWS:ncs?ncs.whiteSpace:'', nameOW:ncs?ncs.overflowWrap:'', nameText:nm?nm.textContent.trim():''};
  });
  const docOverflow = document.documentElement.scrollWidth > document.documentElement.clientWidth + 1;
  return JSON.stringify({vw:vw, feed:fr?{l:Math.round(fr.left),r:Math.round(fr.right),w:Math.round(fr.width),sw:feed.scrollWidth,cw:feed.clientWidth}:null,
    rows:rows, docOverflow:docOverflow, feedOverflow:feed?(feed.scrollWidth>feed.clientWidth+1):false,
    anyRowOverflow:rows.some(function(r){return r.right>vw||r.width>(feed?feed.clientWidth:0)+1;}),
    maxRight:rows.length?Math.max.apply(null,rows.map(function(r){return r.right;})):0});
})()
