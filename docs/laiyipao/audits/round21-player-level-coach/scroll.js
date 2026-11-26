(() => {
  const el=document.querySelector('.r21-level-coach');
  if(el){ el.scrollIntoView({block:'center',behavior:'instant'}); }
  const r=el&&el.getBoundingClientRect();
  return {found:!!el,top:r&&Math.round(r.top),height:r&&Math.round(r.height),text:el?(el.textContent||'').replace(/\s+/g,' ').trim().slice(0,200):''};
})()