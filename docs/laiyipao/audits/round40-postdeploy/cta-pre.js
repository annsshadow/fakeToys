(() => { try {
  const q=s=>document.querySelector(s);
  const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
  const vis=e=>!!e&&e.offsetParent!==null;
  const cta=q(".r40-ai-cta a");
  return JSON.stringify({hash:location.hash,
    ctaVisible:!!cta&&vis(cta),ctaHref:cta?cta.getAttribute("href"):null,
    ratingVisible:vis(q(".r40-rating-mechanics")),
    activeNav:Array.from(document.querySelectorAll(".nav-item.on")).map(txt),err:null});
}catch(e){return JSON.stringify({error:String(e)})}})()