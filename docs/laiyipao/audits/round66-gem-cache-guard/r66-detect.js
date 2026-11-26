(function(){
  var ls = {};
  try { for (var i=0;i<localStorage.length;i++){ var k=localStorage.key(i); ls[k] = (localStorage.getItem(k)||'').slice(0,50); } } catch(e){}
  var card = document.querySelector('.r10-gem-gain');
  var empty = document.querySelector('.r10-gem-empty');
  var rows = [].slice.call(document.querySelectorAll('.r10-gem-row'));
  function txt(e){return e?(e.innerText||e.textContent||'').replace(/\s+/g,' ').trim():'';}
  return JSON.stringify({
    hash: location.hash,
    lsKeys: Object.keys(ls),
    lyp_guest: ls['lyp_guest']||'', lyp_profile: ls['lyp_profile']||'',
    cardText: txt(card), emptyText: txt(empty), rowsCount: rows.length,
    errors: window.__qaErrors||[], bodyLen: (document.body.innerText||'').length
  });
})()
