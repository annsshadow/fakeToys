(() => { try {
 const txt=e=>e?(e.innerText||e.textContent||"").trim():"";
 return JSON.stringify({hash:location.hash,active:Array.from(document.querySelectorAll(".nav-item.on,.nav-sub-item.on")).map(txt)});
}catch(e){return JSON.stringify({error:String(e)})}})()
