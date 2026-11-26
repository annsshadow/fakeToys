(() => {
  const c = e => getComputedStyle(e);
  const pr = s => { const m=(s||"").match(/rgba?\(([^)]+)\)/); if(!m)return null; const p=m[1].split(",").map(Number); return {r:p[0],g:p[1],b:p[2]}; };
  const lum = x => { const f=v=>{v/=255;return v<=0.03928?v/12.92:Math.pow((v+0.055)/1.055,2.4)}; return 0.2126*f(x.r)+0.7152*f(x.g)+0.0722*f(x.b); };
  const o = { hash:location.hash, innerW:innerWidth, docW:document.documentElement.scrollWidth, hOverflow:document.documentElement.scrollWidth-innerWidth };
  const inp = document.querySelector(".preset-input") || document.querySelector("input[placeholder]");
  if (inp) { const ph = inp.querySelector(".ph,.uni-input-placeholder,.input-placeholder") || inp; o.ph = { color:c(ph).color, cls:String(ph.className||""), sw:ph.scrollWidth, cw:ph.clientWidth }; const fg=pr(c(ph).color); if(fg){ const l1=lum(fg),l2=lum({r:26,g:34,b:51}); o.ph.ratio=Math.round(((Math.max(l1,l2)+0.05)/(Math.min(l1,l2)+0.05))*100)/100; } }
  return o;
})()
