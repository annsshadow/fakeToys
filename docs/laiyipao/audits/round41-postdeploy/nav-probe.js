(() => { const q=s=>document.querySelector(s), qa=s=>Array.from(document.querySelectorAll(s));
 const h=q('#reactions'); const p=h?h.parentElement:null;
 return JSON.stringify({hash:location.hash,
  navs:qa('a.nav-item').map(a=>a.getAttribute('href')),
  parentTag:p?p.tagName:null,parentClass:p?p.className:null,
  parentTables:p?qa('.el-table',p).length:null,
  parentRows:p?qa('.el-table',p).map(t=>qa('.el-table__body tbody tr',t).length):null,
  ratingTables:qa('.r40-rating-mechanics .el-table').length,
  ratingRows:qa('.r40-rating-mechanics .el-table__body tbody tr').length});})()