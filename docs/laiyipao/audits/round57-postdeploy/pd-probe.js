(() => {
  const qa=s=>Array.from(document.querySelectorAll(s)); const st=el=>getComputedStyle(el);
  const txt=el=>(el?.textContent||'').trim();
  const out={href:location.href,hash:location.hash,innerW:innerWidth,docW:document.documentElement.scrollWidth};
  out.hOverflow=out.docW-out.innerW;
  // 公告守卫
  const at=qa('.ann-title')[0];
  out.ann=at?{textOverflow:st(at).textOverflow,whiteSpace:st(at).whiteSpace,overflow:st(at).overflow,minWidth:st(at).minWidth,sw:at.scrollWidth,cw:at.clientWidth}:null;
  out.annDates=qa('.ann-date').length;
  out.annLatest=qa('[class*=badge],[class*=latest]').filter(e=>/最新/.test(txt(e))).length;
  // 占位守卫+省略
  const phs=qa('.uni-input-placeholder,.ph,.input-placeholder');
  out.phs=phs.slice(0,6).map(e=>({text:txt(e).slice(0,30)||e.getAttribute('placeholder')||'',color:st(e).color,textOverflow:st(e).textOverflow,whiteSpace:st(e).whiteSpace,overflow:st(e).overflow,minWidth:st(e).minWidth,sw:e.scrollWidth,cw:e.clientWidth}));
  // 精确截断分类
  out.hardTrunc=[]; out.ellipsisOK=[];
  qa('div,span,p,h1,h2,h3,h4,h5,h6,button,view,text,input').forEach(el=>{
    const t=txt(el)||el.getAttribute('placeholder')||''; if(!t) return;
    const s=st(el); if(el.scrollWidth<=el.clientWidth+2||el.clientWidth<=0) return;
    const guarded=s.textOverflow==='ellipsis'&&s.whiteSpace==='nowrap'&&s.overflow==='hidden';
    const rec={cls:(el.className||'').toString().slice(0,30),t:t.slice(0,24),sw:el.scrollWidth,cw:el.clientWidth};
    (guarded?out.ellipsisOK:out.hardTrunc).push(rec);
  });
  out.hardTruncCount=out.hardTrunc.length; out.ellipsisOKCount=out.ellipsisOK.length;
  // 回归锚点
  out.r51radar=qa('.r51-radar').length;
  out.r51rings=qa('.r51-radar-ring').length;
  out.r51axes=qa('.r51-radar-axis').length;
  out.r51honesty=qa('.r51-radar-honesty').map(e=>txt(e)).filter(Boolean);
  out.r53concl=qa('.r3-ai-conclusion,[class*=ai-conclusion]').map(e=>txt(e).slice(0,80));
  out.hasLongFloat=/\d{2}\.\d{10,}/.test(document.body.innerText);
  out.r14cta=qa('.r14-advice-empty-cta').map(e=>({t:txt(e),ws:st(e).whiteSpace}));
  out.r12=qa('.r12-priority-text,.r12-priority-cta').map(e=>txt(e).slice(0,40));
  out.r48self=qa('[class*=cta]').map(e=>txt(e)).filter(x=>/在本页强化/.test(x));
  return out;
})()