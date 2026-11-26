(() => { try {
  const q=(s)=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const sec=q(".r37-mastery-mechanics"); const cta=q(".r37-ai-cta a",sec||document);
  const w=q(".r37-mastery-table-scroll",sec||document); const inner=(w&&q(".el-scrollbar__wrap",w))||w;
  return JSON.stringify({hashBefore:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,rowCount:sec?sec.querySelectorAll(".el-table__body tbody tr").length:0,ctaText:txt(cta),wrapSw:inner?inner.scrollWidth:0,wrapCw:inner?inner.clientWidth:0});
}catch(e){return JSON.stringify({error:String(e)})}})()