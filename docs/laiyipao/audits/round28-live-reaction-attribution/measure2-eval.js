(() => {
  const vw=window.innerWidth;
  const feed=document.querySelector('.reaction-feed');
  if(!feed)return JSON.stringify({noFeed:true,vw:vw});
  const lines=[...document.querySelectorAll('.reaction-line')].map(function(el){
    const r=el.getBoundingClientRect();const cs=getComputedStyle(el);
    const cue=el.querySelector('.reaction-cue');const cr=cue?cue.getBoundingClientRect():null;
    const ccs=cue?getComputedStyle(cue):null;
    return {right:Math.round(r.right),width:Math.round(r.width),sw:el.scrollWidth,cw:el.clientWidth,
      ws:cs.whiteSpace,ov:cs.overflowX,maxW:cs.maxWidth,
      cueRight:cr?Math.round(cr.right):null,cueW:cr?Math.round(cr.width):null,cueSW:cue?cue.scrollWidth:null,cueCW:cue?cue.clientWidth:null,cueWS:ccs?ccs.whiteSpace:null,
      txt:(el.textContent||'').trim().slice(0,20)};
  });
  const ai=document.querySelector('.r28-feed-ai');const ar=ai?ai.getBoundingClientRect():null;
  return JSON.stringify({vw:vw,feedRight:Math.round(feed.getBoundingClientRect().right),feedSW:feed.scrollWidth,feedCW:feed.clientWidth,
    lines:lines,aiRight:ar?Math.round(ar.right):null,aiWS:ai?getComputedStyle(ai).whiteSpace:null,
    settled:!!document.querySelector('.result-title')});
})()
