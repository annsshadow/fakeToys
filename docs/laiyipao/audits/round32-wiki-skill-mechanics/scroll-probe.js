(() => { try {
  const q=s=>document.querySelector(s);
  const wrap=q('.r32-skill-table-scroll .el-scrollbar__wrap');
  const before=wrap?wrap.scrollLeft:0;
  if(wrap) wrap.scrollLeft=wrap.scrollWidth;
  const after=wrap?wrap.scrollLeft:0;
  const lastCol=q('#skill-table .el-table__header th:last-child .cell');
  const rows=[...document.querySelectorAll('#skill-table .el-table__body tbody tr')];
  const lastRow=rows[rows.length-1];
  const lastCell=lastRow?[...lastRow.querySelectorAll('td .cell')].pop():null;
  return JSON.stringify({before,after,sw:wrap?.scrollWidth,cw:wrap?.clientWidth,lastHeader:lastCol?.textContent.trim(),lastCellText:lastCell?.textContent.trim(),scrolledToEnd:after>= (wrap?.scrollWidth??0)-(wrap?.clientWidth??0)-2});
} catch(e){ return JSON.stringify({error:String(e)}) } })()