(function(){var b=[].slice.call(document.querySelectorAll('.btn')).filter(function(e){return(e.innerText||'').indexOf('全部不要了')>=0});
if(b.length){b[0].click();return 'NEXT_WAVE'}return 'NO_MASK'})()