(() => {
  const cta=document.querySelector('.r28-feed-cta');
  const r=cta?cta.getBoundingClientRect():null;
  const el=r?document.elementFromPoint(r.left+r.width/2,r.top+r.height/2):null;
  const stack=r?document.elementsFromPoint(r.left+r.width/2,r.top+r.height/2).slice(0,8).map(e=>({tag:e.tagName,cls:String(e.className||'').slice(0,60),txt:(e.textContent||'').trim().slice(0,30),z:getComputedStyle(e).zIndex})):[];
  return JSON.stringify({url:location.href,result:document.querySelector('.result-title')?.textContent||'',ctaText:cta?.textContent||'',ctaRect:r?{l:r.left,t:r.top,w:r.width,h:r.height}:null,hit:el?{tag:el.tagName,cls:String(el.className||''),txt:(el.textContent||'').trim().slice(0,40)}:null,stack,timeline:!!document.querySelector('.r29-timeline-overlay'),legend:!!document.querySelector('.legend-overlay')});
})()
