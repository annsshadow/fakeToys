(() => { try {
 const q=(s,r)=>(r||document).querySelector(s); const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
 const scroll=q('.r34-level-table-scroll');
 const info={cls:scroll?scroll.className:null, children:scroll?Array.from(scroll.children).map(c=>c.className):[]};
 const wraps=qa('.el-scrollbar__wrap',scroll); info.wraps=wraps.map(w=>({cls:w.className,sw:w.scrollWidth,cw:w.clientWidth,sl:w.scrollLeft,ox:getComputedStyle(w).overflowX,tag:w.tagName}));
 const ths=qa('th',scroll); info.thCount=ths.length; info.ths=ths.map(t=>(t.innerText||t.textContent||'').trim()).slice(0,20);
 const hdr=qa('.el-table__header',scroll); info.headers=hdr.map(h=>({cls:h.className,w:h.scrollWidth,cw:h.clientWidth}));
 const tables=qa('table',scroll); info.tables=tables.map(t=>({cls:t.className,w:t.scrollWidth,rows:t.querySelectorAll('tbody tr').length}));
 return JSON.stringify(info);
}catch(e){return JSON.stringify({error:String(e)})}})()