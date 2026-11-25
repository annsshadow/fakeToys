(() => { try {
 const qa=(s,r)=>Array.from((r||document).querySelectorAll(s)), q=(s,r)=>(r||document).querySelector(s), txt=e=>e?(e.innerText||e.textContent||'').trim():'';
 const vis=s=>{const e=q(s);return !!e&&e.offsetParent!==null};
 const rows=(s,r)=>{const e=q(s,r);return e?qa('.el-table__body tbody tr',e).length:null};
 const secs=qa('section.wiki-section').filter(e=>e.offsetParent!==null);
 const de=document.documentElement;
 return JSON.stringify({hash:location.hash,visibleSections:secs.map(e=>e.className).slice(0,4),
  ratingVisible:!!q('.r40-rating-mechanics')&&q('.r40-rating-mechanics').offsetParent!==null,
  ratingRows:rows('.r40-rating-mechanics'),
  reactionRows:rows('#reactions'),reaction150:txt(document.body).includes('150‰'),reaction4px:txt(document.body).includes('4px'),
  tables:qa('.el-table').filter(t=>t.offsetParent!==null).map(t=>qa('.el-table__body tbody tr',t).length),
  chapterRows:rows('.r41-chapter-mechanics'),levelRows:rows('.r34-level-mechanics'),
  levelChapterRows:rows('.r34-level-table-scroll'),
  r42Note:!!q('.r42-level-chapter-note')&&q('.r42-level-chapter-note').offsetParent!==null,
  r42Text:q('.r42-level-chapter-note')?txt(q('.r42-level-chapter-note')).slice(0,120):null,
  docW:de.scrollWidth,innerW:innerWidth,dirty:/NaN|undefined|\[object Object\]/.test(txt(document.body)),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()