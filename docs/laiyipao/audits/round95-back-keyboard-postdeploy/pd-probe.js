(()=>{
  var out={url:location.href,hash:location.hash,title:document.title,bodyLen:document.body?document.body.innerText.length:0};
  try{out.gcp=(typeof getCurrentPages==='function')?getCurrentPages().length:'NA'}catch(e){out.gcp='ERR:'+String(e)}
  var b=document.querySelector('.back-btn');
  out.backExists=!!b;
  if(b){out.backRole=b.getAttribute('role');out.backTab=b.getAttribute('tabindex');out.backText=(b.textContent||'').trim();out.backTag=b.tagName;out.backCls=String(b.className||'');out.aria=b.getAttribute('aria-label')}
  out.isLogin=/login/i.test(location.hash)||(document.body&&/登录/.test(document.body.innerText.slice(0,120)));
  try{out.iw=window.innerWidth;out.cw=document.documentElement.clientWidth;out.sw=document.documentElement.scrollWidth;out.hOverflow=out.sw>out.iw+1}catch(e){}
  var grid=[].slice.call(document.querySelectorAll('.grid-item'));
  out.gridLabels=grid.map(function(g){return (g.innerText||'').replace(/\s+/g,' ').trim().slice(0,20)});
  return JSON.stringify(out);
})()
