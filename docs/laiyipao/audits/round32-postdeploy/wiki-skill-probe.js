(() => { try {
  const q = s => document.querySelector(s);
  const table = q('#skill-table');
  const headers = table ? [...table.querySelectorAll('.el-table__header th .cell')].map(x=>x.textContent.trim()) : [];
  const rows = table ? [...table.querySelectorAll('.el-table__body tbody tr')].map(tr => [...tr.querySelectorAll('td .cell')].map(td=>td.textContent.trim())) : [];
  const ai = q('.r32-ai-note');
  const txt = ai ? ai.textContent : '';
  const cta = q('.r32-ai-cta a');
  const scroll = q('.r32-skill-table-scroll');
  const wrap = q('.r32-skill-table-scroll .el-scrollbar__wrap');
  const de = document.documentElement;
  return JSON.stringify({ url: location.href, headers, rowCount: rows.length, rows,
    aiText: txt, hasAI: txt.includes('AI'), conclusion: q('.r32-ai-conclusion')?.textContent?.trim() || '',
    basis: q('.r32-ai-basis')?.textContent?.trim() || '', ctaText: cta?.textContent?.trim() || '', ctaHref: cta?.getAttribute('href') || '',
    honesty: q('.r32-ai-honesty')?.textContent?.trim() || '',
    hasBad: /NaN|undefined|\[object Object\]|\{total\}|\{active\}|\{passive\}|\{multi\}/.test(document.body.innerText),
    docW: de.scrollWidth, clientW: de.clientWidth, innerW: innerWidth,
    scrollSw: scroll?.scrollWidth || 0, scrollCw: scroll?.clientWidth || 0,
    wrapSw: wrap?.scrollWidth || 0, wrapCw: wrap?.clientWidth || 0, wrapLeft: wrap?.scrollLeft || 0,
    hasComposite: !!q('#composite'), hash: location.hash });
} catch(e){ return JSON.stringify({error:String(e)}) } })()