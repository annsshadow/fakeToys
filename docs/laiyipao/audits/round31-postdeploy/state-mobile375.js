(() => {
  const de=document.documentElement;
  const w=document.querySelector(".el-table__body-wrapper"); if(w) w.scrollLeft=0;
  window.scrollTo(0, Math.min(180, Math.max(0,de.scrollHeight-window.innerHeight)));
  return JSON.stringify({state:"mobile375-table-reset", scrollY:window.scrollY, tableLeft:w?w.scrollLeft:null});
})()
