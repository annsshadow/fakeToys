(() => {
  var s=window.__R30_STRESS__;
  if(!s||!s.applied)return JSON.stringify({restored:false});
  var nm=document.querySelector('.reaction-name');var at=document.querySelector('.r28-attribution');
  if(nm)nm.textContent=s.name;if(at)at.textContent=s.attr;
  s.applied=false;
  return JSON.stringify({restored:true,name:s.name.slice(0,40),attr:s.attr.slice(0,40)});
})()
