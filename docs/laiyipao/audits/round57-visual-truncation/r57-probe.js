(() => {
  const qa = s => Array.from(document.querySelectorAll(s));
  const cs = el => getComputedStyle(el);
  const txt = el => (el?.textContent||'').trim();
  const rect = el => { if(!el) return null; const r=el.getBoundingClientRect(); return {x:Math.round(r.x),y:Math.round(r.y),w:Math.round(r.width),h:Math.round(r.height),right:Math.round(r.right),bottom:Math.round(r.bottom)}; };
  const parseRGB = s => { const t=(s||''); if(t.indexOf('rgb')<0) return null; const a=t.slice(t.indexOf('(')+1,t.indexOf(')')).split(',').map(Number); return {r:a[0],g:a[1],b:a[2],a:a.length>3?a[3]:1}; };
  const lum = c => { const f=v=>{v/=255;return v<=0.03928?v/12.92:Math.pow((v+0.055)/1.055,2.4)}; return 0.2126*f(c.r)+0.7152*f(c.g)+0.0722*f(c.b); };
  const bgOf = el => { let e=el; while(e){ const c=parseRGB(cs(e).backgroundColor); if(c&&c.a>0.5) return c; e=e.parentElement;} return {r:255,g:255,b:255,a:1}; };
  const contrast2 = (fg,bg) => { const l1=lum(fg),l2=lum(bg),hi=Math.max(l1,l2),lo=Math.min(l1,l2); return Math.round(((hi+0.05)/(lo+0.05))*100)/100; };
  const directText = el => Array.from(el.childNodes).some(n=>n.nodeType===3&&n.textContent.trim());
  const innerW = innerWidth, docW = document.documentElement.scrollWidth, bodyW = document.body.scrollWidth;
  const out = {href:location.href,hash:location.hash,innerW,docW,bodyW,hOverflow:docW-innerW,bodyLen:document.body.innerText.length};
  const dateEls = qa('.ann-date');
  out.ann = {dateCount:dateEls.length,
    date0: dateEls[0]?{text:txt(dateEls[0]),rect:rect(dateEls[0]),color:cs(dateEls[0]).color}:null,
    latestCount: qa('[class*=badge],[class*=latest]').filter(e=>/最新/.test(txt(e))).length,
    head: qa('.ann-head')[0]?{flexWrap:cs(qa('.ann-head')[0]).flexWrap,rect:rect(qa('.ann-head')[0])}:null,
    title0: qa('.ann-title')[0]?{text:txt(qa('.ann-title')[0]).slice(0,40),rect:rect(qa('.ann-title')[0]),minWidth:cs(qa('.ann-title')[0]).minWidth,overflow:cs(qa('.ann-title')[0]).overflow,textOverflow:cs(qa('.ann-title')[0]).textOverflow,whiteSpace:cs(qa('.ann-title')[0]).whiteSpace,sw:qa('.ann-title')[0].scrollWidth,cw:qa('.ann-title')[0].clientWidth}:null,
    rightOver: qa('*').filter(e=>{const r=e.getBoundingClientRect();return r.width>0&&r.right>innerW+1;}).slice(0,12).map(e=>({cls:(e.className||'').toString().slice(0,40),tag:e.tagName,right:Math.round(e.getBoundingClientRect().right),t:txt(e).slice(0,20)}))};
  const inputs=qa('input'), phs=qa('.uni-input-placeholder,.ph,.input-placeholder');
  out.ene={inputCount:inputs.length,
    inputs:inputs.slice(0,4).map(e=>({cls:(e.className||'').toString().slice(0,40),placeholder:e.getAttribute('placeholder')||'',rect:rect(e)})),
    phs:phs.slice(0,6).map(e=>{const st=cs(e),bg=bgOf(e),fg=parseRGB(st.color)||{r:0,g:0,b:0};return {cls:(e.className||'').toString().slice(0,40),text:txt(e).slice(0,40)||e.getAttribute('placeholder')||'',color:st.color,bg:bg,ratio:contrast2(fg,bg),sw:e.scrollWidth,cw:e.clientWidth,overflow:st.overflow,textOverflow:st.textOverflow,whiteSpace:st.whiteSpace,minWidth:st.minWidth,rect:rect(e)};})};
  const trunc=[],low=[];
  qa('div,span,p,h1,h2,h3,h4,h5,h6,button,view,text,input').forEach(el=>{ if(!directText(el)) return; const st=cs(el); if(el.scrollWidth>el.clientWidth+2&&el.clientWidth>0&&(st.overflow==='hidden'||st.textOverflow==='ellipsis'||st.whiteSpace==='nowrap')) trunc.push({cls:(el.className||'').toString().slice(0,40),t:txt(el).slice(0,30)||el.getAttribute('placeholder')||'',sw:el.scrollWidth,cw:el.clientWidth}); });
  qa('div,span,p,h1,h2,h3,h4,h5,h6,button').forEach(el=>{ const t=txt(el); if(t.length<2||t.length>80||!directText(el)) return; const r=el.getBoundingClientRect(); if(!r.width||!r.height) return; const st=cs(el); if(st.visibility==='hidden'||st.display==='none'||st.opacity==='0') return; const fg=parseRGB(st.color); if(!fg) return; const c=contrast2(fg,bgOf(el)); if(c<4.5) low.push({cls:(el.className||'').toString().slice(0,40),t:t.slice(0,24),ratio:c,size:st.fontSize}); });
  out.truncCount=trunc.length; out.trunc=trunc.slice(0,10); out.lowContrastCount=low.length; out.lowContrast=low.slice(0,10);
  out.ctas=qa('[class*=cta],[class*=go],[class*=btn]').filter(e=>{const r=e.getBoundingClientRect();return r.width&&r.height;}).slice(0,20).map(e=>({cls:(e.className||'').toString().slice(0,40),t:txt(e).slice(0,24)}));
  return out;
})()