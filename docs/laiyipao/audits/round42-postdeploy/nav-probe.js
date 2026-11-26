(() => { try {
 const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)), q=(s,r)=>(r||document).querySelector(s);
 const txt=e=>((e&&(e.innerText||e.textContent))||'').trim();
 const navs=qa('a.nav-item').map(a=>({href:a.getAttribute('href'),text:txt(a)}));
 const tables=qa('.el-table').filter(t=>t.offsetParent!==null).map(t=>qa('.el-table__body tbody tr',t).length);
 return JSON.stringify({hash:location.hash,navs,tables,docW:document.documentElement.scrollWidth,innerW:innerWidth,dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()