JSON.stringify({
  url: location.href,
  links: Array.from(document.querySelectorAll(".link")).map(function(e){return {tag:e.tagName,text:(e.textContent||"").trim(),role:e.getAttribute("role"),tab:e.getAttribute("tabindex"),tabIndex:e.tabIndex,cursor:getComputedStyle(e).cursor}}),
  hasCopy: document.body.textContent.indexOf("复制")>=0,
  cw: document.documentElement.clientWidth, sw: document.documentElement.scrollWidth
})
