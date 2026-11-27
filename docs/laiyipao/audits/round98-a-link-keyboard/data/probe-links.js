JSON.stringify({
 url: location.href,
 links: Array.from(document.querySelectorAll(".link")).map(function(e){return {tag:e.tagName,text:(e.textContent||"").trim().slice(0,22),role:e.getAttribute("role"),tab:e.getAttribute("tabindex"),tabIndex:e.tabIndex,cursor:getComputedStyle(e).cursor,w:Math.round(e.getBoundingClientRect().width),h:Math.round(e.getBoundingClientRect().height)}}),
 cw: document.documentElement.clientWidth, sw: document.documentElement.scrollWidth,
 hOverflow: document.documentElement.scrollWidth>document.documentElement.clientWidth
})
