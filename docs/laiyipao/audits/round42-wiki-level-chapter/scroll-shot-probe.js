(() => { try {
 const q=(s,r)=>(r||document).querySelector(s);
 const note=q('.r42-level-chapter-note'); const scroll=q('.r34-level-table-scroll');
 if(note) note.scrollIntoView({block:'center'});
 const wrap=scroll?q('.el-scrollbar__wrap',scroll):null;
 if(wrap) wrap.scrollLeft=Math.max(0,wrap.scrollWidth-wrap.clientWidth);
 return JSON.stringify({hash:location.hash,scrolled:!!note,wrapSw:wrap?wrap.scrollWidth:null,wrapCw:wrap?wrap.clientWidth:null,wrapSl:wrap?wrap.scrollLeft:null,docW:document.documentElement.scrollWidth,innerW:innerWidth});
}catch(e){return JSON.stringify({error:String(e)})}})()