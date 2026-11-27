JSON.stringify({
 tag: document.activeElement ? document.activeElement.tagName : "",
 cls: document.activeElement ? (document.activeElement.className||"") : "",
 txt: document.activeElement ? (document.activeElement.textContent||"").trim().slice(0,20) : "",
 isLink: !!(document.activeElement && document.activeElement.classList && document.activeElement.classList.contains("link"))
})
