(function(){
  var a=document.querySelector('#app');
  return JSON.stringify({
    url:location.href,ready:document.readyState,
    appHTML:a?a.innerHTML.slice(0,200):'NO_APP',
    bodyLen:(document.body.innerText||'').length,
    scripts:document.querySelectorAll('script').length,
    hasToken:(localStorage.getItem('lyp_token')||'').length
  })
})()