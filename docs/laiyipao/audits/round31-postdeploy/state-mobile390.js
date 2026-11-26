(() => {
  const de=document.documentElement;
  window.scrollTo(0, Math.max(0,de.scrollHeight-window.innerHeight));
  const inp=document.querySelector("input"); if(inp) inp.focus();
  return JSON.stringify({state:"mobile390-bottom", scrollY:window.scrollY, focused:!!inp});
})()
