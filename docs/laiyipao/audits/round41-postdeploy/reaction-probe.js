(() => { const q=s=>document.querySelector(s), qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
 const h=q('#reactions'); let e=h?h.nextElementSibling:null, found=null, hops=[];
 let k=0; while(e && k<8){hops.push(e.tagName+'.'+(typeof e.className==='string'?e.className:''));
  if(e.classList && e.classList.contains('el-table')){found=e;break;}
  const inner=e.querySelector?e.querySelector('.el-table'):null; if(inner){found=inner;break;}
  e=e.nextElementSibling;k++;}
 const rows=found?qa('.el-table__body tbody tr',found).length:null;
 return JSON.stringify({hash:location.hash,hops:hops,found:!!found,rows:rows,
  firstCells:found?qa('.el-table__body tbody tr',found).slice(0,3).map(tr=>tr.innerText.trim().slice(0,40)):[]});})()