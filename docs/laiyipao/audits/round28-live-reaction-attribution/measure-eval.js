(() => {
  const feed=document.querySelector('.reaction-feed');
  if(!feed) return JSON.stringify({noFeed:true,state:(document.querySelector('.result-title')?'SETTLED':'WAIT')});
  const fr=feed.getBoundingClientRect();
  const items=[...feed.querySelectorAll('*')].map(function(el){
    const r=el.getBoundingClientRect();
    return {cls:String(el.className||'').slice(0,48),sw:el.scrollWidth,cw:el.clientWidth,l:Math.round(r.left),rt:Math.round(r.right),w:Math.round(r.width),txt:(el.textContent||'').trim().slice(0,26)};
  }).filter(function(x){return x.sw>x.cw+1||x.rt>fr.right+1});
  const cta=document.querySelector('.r28-feed-cta');
  let hit=null,ctaRect=null;
  if(cta){const r=cta.getBoundingClientRect();ctaRect={l:Math.round(r.left),t:Math.round(r.top),w:Math.round(r.width),h:Math.round(r.height)};const el=document.elementFromPoint(r.left+r.width/2,r.top+r.height/2);hit=el?String(el.className||el.tagName).slice(0,60):null;}
  return JSON.stringify({feedW:Math.round(fr.width),feedL:Math.round(fr.left),feedR:Math.round(fr.right),feedSW:feed.scrollWidth,feedCW:feed.clientWidth,overItems:items.slice(0,12),ctaRect:ctaRect,ctaHit:hit,legend:!!document.querySelector('.legend-overlay'),result:!!document.querySelector('.result-title')});
})()
