(() => { const m=document.querySelector('.r7-compare-mask'); const t=document.querySelector('.r7-compare-title');
  const rows=document.querySelectorAll('.r7-vs-row'); const btns=document.querySelectorAll('.r7-btn');
  return JSON.stringify({mask:!!m,title:t?t.textContent.trim():'',rows:rows.length,
    rowText:rows.length?rows[0].textContent.trim().slice(0,160):'',btns:Array.from(btns).map(b=>b.textContent.trim())});
})()