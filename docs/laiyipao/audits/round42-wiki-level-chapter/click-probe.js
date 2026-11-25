(() => { try {
 const q=(s,r)=>(r||document).querySelector(s); const qa=(s,r)=>Array.from((r||document).querySelectorAll(s));
 const scroll=q('.r34-level-table-scroll'); const rows=qa('.el-table__body tbody tr',scroll); const first=rows[0];
 const link=first?q('td:last-child a',first):null; const chapter=q('.r41-chapter-mechanics');
 const wrap=q('.el-scrollbar__wrap',scroll);
 return JSON.stringify({hash:location.hash,linkText:link?(link.innerText||link.textContent||'').trim():null,linkHref:link?link.getAttribute('href'):null,
  chapterVisible:!!chapter&&chapter.offsetParent!==null,chapterTop:chapter?Math.round(chapter.getBoundingClientRect().top):null,
  scrollTop:wrap?wrap.scrollTop:null,scrollLeft:wrap?wrap.scrollLeft:null,docW:document.documentElement.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()