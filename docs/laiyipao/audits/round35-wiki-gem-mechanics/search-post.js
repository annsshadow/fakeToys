(() => { try {
  const q=(s)=>document.querySelector(s); const qa=(s)=>Array.from(document.querySelectorAll(s));
  const txt=e=>e?(e.innerText||e.textContent||'').trim():"";
  const body=q(".wiki-body");
  return JSON.stringify({hash:location.hash,scrollY:Math.round(scrollY),focused:[...document.querySelectorAll("[data-focus]")].map(x=>x.getAttribute("data-focus")).filter(Boolean).slice(-4),inputValue:q("input[type=search]")?.value||"",highlighted:qa(".row-focus,.is-focus,[data-focus] .el-table__row--focus").length,gemSection:!!q(".r35-gem-mechanics")&&q(".r35-gem-mechanics").offsetParent!==null,activeNav:qa(".nav-item.on").map(txt),bodyHasFeng:document.body.innerText.includes("锋锐石"),bodyHasGold:document.body.innerText.includes("gold")});
}catch(e){return JSON.stringify({error:String(e)})}})()
