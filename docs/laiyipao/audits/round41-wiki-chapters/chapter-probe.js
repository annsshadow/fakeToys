(() => { try {
const qa=s=>Array.from(document.querySelectorAll(s)); const q=s=>document.querySelector(s);
const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
const ROOT=".r41-chapter-table-scroll";
const sec=q(".r41-chapter-mechanics"); const wrap=q(ROOT);
const rows=qa(ROOT+" .el-table__body tbody tr").map(r=>Array.from(r.querySelectorAll(":scope > td > .cell")).map(txt));
const headers=qa(ROOT+" .el-table__header th .cell").map(txt);
const ai={conclusion:txt(q(".r41-ai-conclusion")),basis:txt(q(".r41-ai-basis")),cta:txt(q(".r41-ai-cta")),honesty:txt(q(".r41-ai-honesty")),ctaHref:q(".r41-ai-cta a")?q(".r41-ai-cta a").getAttribute("href"):null};
const scrollWrap=q(ROOT+" .el-scrollbar__wrap");
const before=scrollWrap?scrollWrap.scrollLeft:null; if(scrollWrap) scrollWrap.scrollLeft=9999; const after=scrollWrap?scrollWrap.scrollLeft:null; if(scrollWrap) scrollWrap.scrollLeft=before;
const sel=q(".el-select"); const active=sel?txt(sel):null;
const focused=q(".r41-chapter-mechanics tr.row-focus");
const allText=rows.flat().join(" ")+ai.conclusion+ai.basis+ai.honesty;
const dirty=/NaN|undefined|\[object Object\]|TODO|占位|待补充/.test(allText);
return JSON.stringify({hash:location.hash,sectionVisible:!!sec&&sec.offsetParent!==null,headers,rows,ai,activeChapter:active,focusRow:focused?txt(focused):null,hasFocus:!!focused,scroll:{sw:scrollWrap?scrollWrap.scrollWidth:null,cw:scrollWrap?scrollWrap.clientWidth:null,sl:before,after,ox:scrollWrap?getComputedStyle(scrollWrap).overflowX:null,outerSw:wrap?wrap.scrollWidth:null,outerCw:wrap?wrap.clientWidth:null},page:{docW:document.documentElement.scrollWidth,innerW:innerWidth},dirty});
}catch(e){return JSON.stringify({error:String(e)})}})()
