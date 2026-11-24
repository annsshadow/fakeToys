(() => {
  const de=document.documentElement;
  window.scrollTo(0, 0);
  const wraps=Array.from(document.querySelectorAll(".el-scrollbar__wrap"));
  const w=wraps.find(e=>getComputedStyle(e).overflowX==="auto" && e.scrollWidth>e.clientWidth);
  if(w) w.scrollLeft=0;
  return JSON.stringify({state:"deeplink-reset", scrollY:window.scrollY, wrapLeft:w?w.scrollLeft:null});
})()
