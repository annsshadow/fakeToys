(function() { try {
  var rows = [].slice.call(document.querySelectorAll('tbody tr'));
  var ids = rows.map(function(r){ var t=(r.innerText||'').replace(/\s+/g,' ').trim(); return t.slice(0,120); });
  return JSON.stringify({href:location.href,rowCount:rows.length,rows:ids.slice(0,8),bodyLen:(document.body.innerText||'').length,isLogin:location.href.indexOf('/admin/login')>=0});
 } catch(e) { return JSON.stringify({error:String(e)}); } })()