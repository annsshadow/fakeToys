(() => { try {
 const q=(s,r)=>(r||document).querySelector(s); const note=q('.r42-level-chapter-note'); const a=note&&q('.r42-ai-cta a',note); const ch=q('.r41-chapter-mechanics');
 return JSON.stringify({hash:location.hash,onWiki:location.hash.includes('/wiki'),ctaText:a?(a.innerText||a.textContent||'').trim():null,ctaHref:a?a.getAttribute('href'):null,
  chapterVisible:!!ch&&ch.offsetParent!==null,chapterTop:ch?Math.round(ch.getBoundingClientRect().top):null,docW:document.documentElement.scrollWidth,innerW:innerWidth,err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()