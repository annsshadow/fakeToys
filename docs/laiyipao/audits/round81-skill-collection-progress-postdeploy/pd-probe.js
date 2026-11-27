(() => {
  const $ = (s) => document.querySelector(s);
  const $$ = (s) => Array.from(document.querySelectorAll(s));
  const kpi = $('.r3-kpi');
  const prog = $('.r3-progress-row');
  const filter = $('.r3-filter');
  const label = $('.r3-progress-label');
  const num = $('.r3-progress-num');
  const fill = $('.r3-progress-fill');
  const matrix = $('.skill-matrix');
  const covered = $$('.sm-cell.covered').length;
  const cells = $$('.sm-cell').length;
  const titleHit = $$('.card-title').some(e => (e.textContent||'').indexOf('元素 × 类型覆盖') > -1);
  const chips = $$('.r3-filter .r3-chip').length;
  const activeChip = $$('.r3-filter .r3-chip.active').map(e => (e.textContent||'').trim());
  const resultCount = $$('.result-count').map(e => (e.textContent||'').trim()).filter(t => t.indexOf('显示') > -1);
  const afterKpi = !!(kpi && prog && (kpi.compareDocumentPosition(prog) & 4));
  const beforeFilter = !!(prog && filter && (prog.compareDocumentPosition(filter) & 4));
  const progText = prog ? (prog.textContent||'').trim() : null;
  const labelText = label ? (label.textContent||'').trim() : null;
  const numText = num ? (num.textContent||'').trim() : null;
  const fillStyleWidth = fill ? fill.getAttribute('style') : null;
  const fillComputedWidth = fill ? window.getComputedStyle(fill).width : null;
  const trackWidth = (fill && fill.parentElement) ? window.getComputedStyle(fill.parentElement).width : null;
  let loggedGuest = false;
  try { loggedGuest = !!(window.localStorage.getItem('lyp_token') || window.localStorage.getItem('lyp_token_v2')); } catch (e) {}
  const de = document.documentElement;
  const iw = window.innerWidth, cw = de.clientWidth, sw = de.scrollWidth;
  const hOverflow = !(iw === cw && sw <= cw + 1);
  return JSON.stringify({href:location.href,loggedGuest:loggedGuest,afterKpi:afterKpi,beforeFilter:beforeFilter,progText:progText,labelText:labelText,numText:numText,fillStyleWidth:fillStyleWidth,fillComputedWidth:fillComputedWidth,trackWidth:trackWidth,matrix:!!matrix,covered:covered,cells:cells,titleHit:titleHit,chips:chips,activeChip:activeChip,resultCount:resultCount,iw:iw,cw:cw,sw:sw,hOverflow:hOverflow,bodyLen:document.body?document.body.innerText.length:0});
})()
