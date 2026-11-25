(() => { try {
 const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)), q=(s,r)=>(r||document).querySelector(s), txt=e=>e?(e.innerText||e.textContent||'').trim():'';
 const basics=q('section.wiki-section'); const bt=basics?qa('.el-table',basics):[];
 const comp=q('#composite'); const ct=comp?qa('.el-table',comp):[];
 return JSON.stringify({hash:location.hash,
  reactionTables:bt.map(t=>qa('.el-table__body tbody tr',t).length),
  reactionHas150:txt(basics).includes('150‰'), reactionHas4px:txt(basics).includes('4px'),
  compositeTables:ct.length, compositeRows:ct.map(t=>qa('.el-table__body tbody tr',t).length),
  compositeText:comp?txt(comp).slice(0,160):null, err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()