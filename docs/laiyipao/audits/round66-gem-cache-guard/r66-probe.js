(function(){
  function txt(e){return e?(e.innerText||e.textContent||'').replace(/\s+/g,' ').trim():'';}
  var card=document.querySelector('.r10-gem-gain');
  var empty=document.querySelector('.r10-gem-empty');
  var gemText=document.querySelector('.r10-gem-text');
  var rows=[].slice.call(document.querySelectorAll('.r10-gem-row'));
  var rowsDetail=rows.map(function(r){return txt(r);});
  var cacheKeys=[]; try{ for(var i=0;i<localStorage.length;i++){var k=localStorage.key(i); if(k&&k.indexOf('lyp:r10:gem-gain:')===0) cacheKeys.push([k,(localStorage.getItem(k)||'').slice(0,30)]);} }catch(e){}
  var de=document.documentElement;
  return JSON.stringify({
    hash:location.hash,
    loggedGuest: !!(window.localStorage && localStorage.getItem('lyp_guest')),
    cardText: txt(card), gemText: txt(gemText), emptyText: txt(empty),
    rowsCount: rows.length, rowsDetail: rowsDetail,
    cacheKeys: cacheKeys,
    errors: window.__qaErrors||[],
    innerW: window.innerWidth, docW: de.scrollWidth, clientW: de.clientWidth,
    hOverflow: de.scrollWidth > de.clientWidth,
    bodyLen: (document.body.innerText||'').length
  });
})()
