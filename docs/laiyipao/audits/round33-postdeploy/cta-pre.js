(() => { try {
  const q=s=>document.querySelector(s);
  const cta=q('.r33-ai-cta a');const r=cta?cta.getBoundingClientRect():null;
  return JSON.stringify({hashBefore:location.hash,ctaText:cta?cta.textContent.trim():'',ctaHref:cta?(cta.getAttribute('href')||''):'',ctaRect:r?{l:Math.round(r.left),t:Math.round(r.top),w:Math.round(r.width),h:Math.round(r.height)}:null,enemyTableBefore:!!q('.r33-enemy-mechanics'),rowCount:document.querySelectorAll('.r33-enemy-mechanics .el-table__body tbody tr').length});
}catch(e){return JSON.stringify({error:String(e)})}})()