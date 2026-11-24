(() => {
  const sel='.r29-result-timeline-cta';
  const e=document.querySelector(sel);
  const sheet=e&&e.closest('[class*=result]');
  const b=e?e.getBoundingClientRect():null;
  let hit=null;
  if(b){const x=b.left+b.width/2,y=b.top+b.height/2;const t=document.elementFromPoint(x,y);hit=t?{tag:t.tagName,cls:String(t.className||'').slice(0,90),isCta:!!(t.closest&&t.closest(sel))}:null;}
  const cs=e?getComputedStyle(e):null;
  const rr=document.querySelector('.result-title');
  return JSON.stringify({exists:!!e,text:e?e.textContent.trim():'',inSheet:!!sheet,box:b?{l:Math.round(b.left),t:Math.round(b.top),w:Math.round(b.width),h:Math.round(b.height)}:null,hit,display:cs?cs.display:'',visibility:cs?cs.visibility:'',pointerEvents:cs?cs.pointerEvents:'',zIndex:cs?cs.zIndex:'',result:rr?rr.textContent.trim():''});
})()
