(() => { const q=s=>document.querySelector(s), qa=s=>Array.from(document.querySelectorAll(s));
 const cnt=(root,sel)=>{const e=q(root); return e?qa(sel,e).length:null;};
 return JSON.stringify({hash:location.hash,
  levelsTables:qa('.r34-level-mechanics .el-table').length,
  levelsRows:qa('.r34-level-mechanics .el-table__body tbody tr').length,
  levelsFirst:qa('.r34-level-mechanics .el-table').map(t=>qa('.el-table__body tbody tr',t).length),
  compositeAll:cnt('.r36-composite-mechanics','.el-table__body tbody tr'),
  compositeDirect:cnt('.r36-composite-mechanics','.r36-composite-table-scroll .el-table__body tbody tr'),
  recipeAll:cnt('.r38-recipe-mechanics','.el-table__body tbody tr'),
  recipeDirect:cnt('.r38-recipe-mechanics','.r38-recipe-table-scroll .el-table__body tbody tr')});})()