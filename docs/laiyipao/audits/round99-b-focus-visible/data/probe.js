(() => {
  const el = document.activeElement;
  if (!el || el === document.body) return '{"tag":"BODY"}';
  const cs = getComputedStyle(el);
  const o = {
    tag: el.tagName,
    cls: String(el.className || '').slice(0,70),
    role: el.getAttribute('role'),
    tab: el.getAttribute('tabindex'),
    txt: String(el.textContent || '').trim().slice(0,26),
    ov: cs.outlineStyle + '|' + cs.outlineWidth + '|' + cs.outlineColor,
    ooff: cs.outlineOffset,
    bs: cs.boxShadow === 'none' ? 'none' : cs.boxShadow.slice(0,60),
    bc: cs.borderColor,
    fv: el.matches(':focus-visible')
  };
  return JSON.stringify(o);
})()
