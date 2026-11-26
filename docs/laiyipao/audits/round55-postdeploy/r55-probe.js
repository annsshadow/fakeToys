(() => {
  const qa = s => Array.from(document.querySelectorAll(s));
  const cs = el => getComputedStyle(el);
  const txt = el => (el?.textContent||'').trim();
  const rect = el => { if(!el) return null; const r=el.getBoundingClientRect(); return {x:Math.round(r.x),y:Math.round(r.y),w:Math.round(r.width),h:Math.round(r.height),right:Math.round(r.right),bottom:Math.round(r.bottom)}; };
  const parseRGB = s => { const m=(s||'').match(/rgba?\(([^)]+)\)/); if(!m) return null; const p=m[1].split(',').map(Number); return {r:p[0],g:p[1],b:p[2],a:p.length>3?p[3]:1}; };
  const lum = c => { const f=v=>{v/=255;return v<=0.03928?v/12.92:Math.pow((v+0.055)/1.055,2.4)}; return 0.2126*f(c.r)+0.7152*f(c.g)+0.0722*f(c.b); };
  const bgOf = el => { let e=el; while(e){ const c=parseRGB(cs(e).backgroundColor); if(c&&c.a>0.5) return c; e=e.parentElement;} return {r:255,g:255,b:255,a:1}; };
  const ratio = (fg,bg) => { if(!fg||!bg) return null; const l1=lum(fg),l2=lum(bg); return Math.round(((Math.max(l1,l2)+0.05)/(Math.min(l1,l2)+0.05))*100)/100; };
  const rgbStr = c => 'rgb('+c.r+', '+c.g+', '+c.b+')';
  const out={href:location.href,hash:location.hash,innerW:innerWidth,docW:document.documentElement.scrollWidth,bodyW:document.body.scrollWidth};
  out.hOverflow=out.docW-out.innerW;
  if(out.hash.indexOf('reactions')>=0){
    out.advice=qa('.r14-advice').map(a=>{
      const e=a.querySelector('.r14-advice-empty');
      const cta=e?e.querySelector('.r14-advice-empty-cta'):null;
      return {
        adviceRect:rect(a), adviceMinW:cs(a).minWidth, adviceMaxW:cs(a).maxWidth,
        empty:e?{rect:rect(e),overflowWrap:cs(e).overflowWrap,
          title:txt(e.querySelector('.r14-advice-empty-title')),
          msg:txt(e.querySelector('.r14-advice-empty-msg')),
          titleRect:rect(e.querySelector('.r14-advice-empty-title')),
          msgRect:rect(e.querySelector('.r14-advice-empty-msg'))
        }:null,
        cta:cta?{text:txt(cta),rect:rect(cta),whiteSpace:cs(cta).whiteSpace,maxWidth:cs(cta).maxWidth,display:cs(cta).display}:null
      };
    });
    out.rows=qa('.r3-data-grid .list-row').slice(0,4).map(row=>({
      flexWrap:cs(row).flexWrap, rect:rect(row),
      tags:Array.from(row.querySelectorAll('.pair-chip .tag')).slice(0,4).map(t=>({t:txt(t),h:rect(t).h,sw:t.scrollWidth,cw:t.clientWidth})),
      stats:Array.from(row.querySelectorAll('.stat')).slice(0,6).map(t=>({t:txt(t),h:rect(t).h,sw:t.scrollWidth,cw:t.clientWidth}))
    }));
    out.readyCta=qa('#r14-advice-action-btn').length;
  }
  if(out.hash.indexOf('bag')>=0){
    const inp=qa('input.preset-input')[0] || qa('input[placeholder="预设名（最多 20 字）"]')[0];
    if(inp){
      const phEl=inp.querySelector('.uni-input-placeholder, .ph, .input-placeholder');
      const el=phEl||inp;
      out.preset={
        placeholder:inp.getAttribute('placeholder'),
        placeholderClass:inp.getAttribute('placeholder-class'),
        maxlength:inp.getAttribute('maxlength'),
        color:cs(el).color, bg:rgbStr(bgOf(el)),
        ratio:ratio(parseRGB(cs(el).color), bgOf(el)),
        phRect:rect(el), phSW:el.scrollWidth, phCW:el.clientWidth,
        inputRect:rect(inp), inputSW:inp.scrollWidth, inputCW:inp.clientWidth
      };
    }
    out.saveHint=txt(qa('.preset-saved-hint')[0]);
    out.presetCount=qa('.preset-item').length;
  }
  return out;
})()
