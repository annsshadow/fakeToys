(() => { try {
const qa=(s)=>Array.from(document.querySelectorAll(s));
const navs=qa(".nav-item, .nav-link, a, button").filter(e=>{const t=(e.innerText||e.textContent||"").trim(); return /核心规则|技能图鉴|合成技能|敌人抗性|关卡一览|宝石图鉴|专精树/.test(t)});
return JSON.stringify(navs.map(e=>({tag:e.tagName,cls:String(e.className||"").slice(0,80),text:(e.innerText||e.textContent||"").trim().slice(0,30),href:e.getAttribute("href")})));
}catch(e){return JSON.stringify({error:String(e)})}})()