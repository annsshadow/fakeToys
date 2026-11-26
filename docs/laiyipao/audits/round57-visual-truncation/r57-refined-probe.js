(() => {
  const qa=s=>Array.from(document.querySelectorAll(s)); const st=el=>getComputedStyle(el);
  const txt=el=>(el?.textContent||'').trim();
  const out={href:location.href,innerW:window.innerWidth,docW:document.documentElement.scrollWidth,hardTrunc:[],ellipsisOK:[]};
  qa('div,span,p,h1,h2,h3,h4,h5,h6,button,view,text,input').forEach(el=>{
    const t=txt(el)||el.getAttribute('placeholder')||''; if(!t) return;
    const s=st(el); if(el.scrollWidth<=el.clientWidth+2||el.clientWidth<=0) return;
    const guarded = s.textOverflow==='ellipsis' && s.whiteSpace==='nowrap' && s.overflow==='hidden';
    const rec={cls:(el.className||'').toString().slice(0,40),t:t.slice(0,30),sw:el.scrollWidth,cw:el.clientWidth,textOverflow:s.textOverflow,whiteSpace:s.whiteSpace,overflow:s.overflow,minWidth:s.minWidth};
    (guarded?out.ellipsisOK:out.hardTrunc).push(rec);
  });
  out.hardTruncCount=out.hardTrunc.length; out.ellipsisOKCount=out.ellipsisOK.length;
  out.phs=qa('.uni-input-placeholder,.ph').map(e=>({cls:(e.className||'').toString(),t:txt(e).slice(0,40),sw:e.scrollWidth,cw:e.clientWidth,textOverflow:st(e).textOverflow,whiteSpace:st(e).whiteSpace,overflow:st(e).overflow,minWidth:st(e).minWidth,color:st(e).color}));
  return out;
})()