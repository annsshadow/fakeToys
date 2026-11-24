(() => {
  const ov=document.querySelector('.r29-timeline-overlay');
  if(!ov) return JSON.stringify({open:false});
  const box=ov.querySelector('.r29-timeline-box')||ov.querySelector('.box')||ov;
  const scroll=ov.querySelector('.r29-timeline-scroll')||ov.querySelector('.scroll');
  const br=box.getBoundingClientRect();
  const rows=[...ov.querySelectorAll('.r29-timeline-row')].map(function(r){
    const rr=r.getBoundingClientRect();
    return {idx:r.getAttribute('data-r30-index'),wave:r.getAttribute('data-r30-wave'),enemy:r.getAttribute('data-r30-enemy'),text:r.textContent.trim().replace(/\s+/g,' ').slice(0,140),right:Math.round(rr.right),w:Math.round(rr.width),sw:r.scrollWidth,cw:r.clientWidth};
  });
  const pauseEls=[...document.querySelectorAll('button,.btn,[class*=pause]')].filter(function(e){return /暂停|继续/.test(e.textContent||'')});
  return JSON.stringify({open:true, box:{l:Math.round(br.left),t:Math.round(br.top),w:Math.round(br.width),h:Math.round(br.height)}, z:getComputedStyle(ov).zIndex, rowCount:rows.length, rows:rows.slice(0,5), lastRows:rows.slice(-3), scroll:{top:scroll?scroll.scrollTop:null,h:scroll?scroll.scrollHeight:null,ch:scroll?scroll.clientHeight:null,overflowY:scroll?getComputedStyle(scroll).overflowY:null}, legend:!!document.querySelector('.legend-overlay'), pauseLabels:pauseEls.map(function(e){return (e.textContent||'').trim().slice(0,20)}), empty:!!ov.querySelector('.r29-timeline-empty'), truncated:!!ov.querySelector('.r29-timeline-truncated'), result:!!document.querySelector('.result-title'), docOverflow:document.documentElement.scrollWidth>document.documentElement.clientWidth+1, hasUndefined:(ov.textContent||'').includes('undefined'), hasNaN:(ov.textContent||'').includes('NaN')});
})()
