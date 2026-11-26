(() => { try { const q=(s)=>document.querySelector(s); const txt=e=>e?(e.innerText||e.textContent||'').trim():''; const cta=q('[data-r35-cta]');
 return JSON.stringify({hashBefore:location.hash,ctaText:cta?txt(cta):'',ctaHref:cta?(cta.getAttribute('href')||''):'',ctaVisible:cta?!!(cta.offsetWidth||cta.offsetHeight):false,gemTableFound:!!q('.r35-gem-mechanics .el-table'),gemRowCount:document.querySelectorAll('.r35-gem-mechanics .el-table__body tbody tr').length});
}catch(e){return JSON.stringify({error:String(e)})}})()
