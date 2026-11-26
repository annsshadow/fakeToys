(() => { try {
  window.scrollTo(0, 900);
  const q=s=>document.querySelector(s);
  const wrap=q('.r34-level-table-scroll .el-scrollbar__wrap');
  if(wrap) wrap.scrollLeft=wrap.scrollWidth;
  return JSON.stringify({hash:location.hash,scrollY:Math.round(window.scrollY),rows:document.querySelectorAll('.r34-level-mechanics .el-table__body tbody tr').length,tableVisible:!!q('.r34-level-mechanics'),wrapSw:wrap?wrap.scrollWidth:0,wrapSl:wrap?Math.round(wrap.scrollLeft):0});
}catch(e){return JSON.stringify({error:String(e)})}})()
