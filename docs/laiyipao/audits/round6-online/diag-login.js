JSON.stringify({
  url:location.href,
  inputs:[].slice.call(document.querySelectorAll("input")).map(function(e){return {type:e.type,placeholder:e.placeholder,value:e.value,cls:e.className}}),
  btnPrimary:[].slice.call(document.querySelectorAll(".btn-primary")).map(function(e){return {text:(e.innerText||"").trim(),cls:e.className}}),
  lsKeys:Object.keys(localStorage),
  hasToken:!!localStorage.getItem("lyp_token")
})