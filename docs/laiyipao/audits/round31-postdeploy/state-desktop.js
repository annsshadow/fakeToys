(() => {
  const de=document.documentElement;
  const max=Math.max(0, de.scrollHeight-window.innerHeight);
  window.scrollTo(0, Math.min(420, max));
  const w=document.querySelector(".el-table__body-wrapper"); if(w) w.scrollLeft=0;
  return JSON.stringify({state:"desktop-scrolled", scrollY:window.scrollY, max, tableLeft:w?w.scrollLeft:null});
})()
