(() => { try {
  const q=s=>document.querySelector(s);
  const wrap=q('.r33-enemy-table-scroll .el-scrollbar__wrap');
  const before=wrap?wrap.scrollLeft:0; if(wrap) wrap.scrollLeft=wrap.scrollWidth; const after=wrap?wrap.scrollLeft:0;
  const lastCol=q('.r33-enemy-mechanics .el-table__header th:last-child .cell');
  const rows=[...document.querySelectorAll('.r33-enemy-mechanics .el-table__body tbody tr')];
  const lastRow=rows[rows.length-1];
  const lastCell=lastRow?[...lastRow.querySelectorAll('td .cell')].pop():null;
  return JSON.stringify({before,after,sw:wrap?wrap.scrollWidth:0,cw:wrap?wrap.clientWidth:0,lastHeader:lastCol?lastCol.textContent.trim():'',lastCellText:lastCell?lastCell.textContent.trim():'',scrolledToEnd:after>=((wrap?wrap.scrollWidth:0)-(wrap?wrap.clientWidth:0)-2)});
}catch(e){return JSON.stringify({error:String(e)})}})()