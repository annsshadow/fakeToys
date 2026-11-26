(() => { try {
  window.scrollTo(0, 900);
  const q=s=>document.querySelector(s);
  const wrap=q('.r33-enemy-table-scroll .el-scrollbar__wrap');
  if(wrap) wrap.scrollLeft=wrap.scrollWidth;
  return JSON.stringify({hash:location.hash,scrollY:Math.round(window.scrollY),rows:document.querySelectorAll('.r33-enemy-mechanics .el-table__body tbody tr').length,tableVisible:!!q('.r33-enemy-mechanics')});
}catch(e){return JSON.stringify({error:String(e)})}})()
