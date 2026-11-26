(function(){
  return {hash:location.hash,bodyLen:document.body.innerText.length,
    scrollY:Math.round(window.scrollY||document.documentElement.scrollTop||0)};
})()