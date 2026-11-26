(() => {
  const qa = (s, r) => Array.from((r || document).querySelectorAll(s));
  const q = (s, r) => (r || document).querySelector(s);
  const txt = e => e ? (e.innerText || e.textContent || '').trim() : '';
  let table = null;
  for (const t of qa('.el-table')) { const sec = t.closest('section'); if (sec && txt(sec).indexOf('反应链一览') >= 0) { table = t; break; } }
  if (!table) table = q('.el-table');
  const headers = qa('.el-table__header th .cell', table).map(txt);
  const rows = qa('.el-table__body tbody tr', table).map(tr => qa('td .cell', tr).map(txt));
  const de = document.documentElement;
  const page = { scrollWidth: de.scrollWidth, clientWidth: de.clientWidth, innerWidth: window.innerWidth, bodyScrollWidth: document.body.scrollWidth };
  const scrollerEls = qa('.el-scrollbar__wrap, .el-table__body-wrapper', table);
  const wrappers = scrollerEls.map(e => ({ cls: String(e.className).slice(0, 60), sw: e.scrollWidth, cw: e.clientWidth, sl: e.scrollLeft, ox: getComputedStyle(e).overflowX }));
  let scrollTest = null;
  const over = scrollerEls.find(e => e.scrollWidth > e.clientWidth + 1);
  if (over) { const before = over.scrollLeft; over.scrollLeft = over.scrollWidth; scrollTest = { before, after: over.scrollLeft, max: over.scrollWidth - over.clientWidth }; }
  const wikiText = txt(q('.wiki'));
  return JSON.stringify({
    url: location.href, title: document.title, hash: location.hash,
    viewport: { w: window.innerWidth, h: window.innerHeight },
    tableFound: !!table, headers, rowCount: rows.length, rows, page, wrappers, scrollTest,
    hasImplement: wikiText.indexOf('层数传播已实现') >= 0,
    hasBurnCloud: wikiText.indexOf('burn_cloud') >= 0 && wikiText.indexOf('仍未实现') >= 0,
    oldWording: wikiText.indexOf('没有特殊效果的只剩两条') >= 0,
    has150permille: wikiText.indexOf('150‰') >= 0,
    has15percent: wikiText.indexOf('15%') >= 0,
    badToken: /NaN|undefined/.test(wikiText)
  });
})()
