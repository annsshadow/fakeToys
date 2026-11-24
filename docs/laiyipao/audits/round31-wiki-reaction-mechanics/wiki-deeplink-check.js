(function(){
  var secs = Array.from(document.querySelectorAll('.wiki-section')).filter(function(s){return getComputedStyle(s).display !== 'none';});
  return JSON.stringify({url:location.href, hash:location.hash, title:document.title,
    visibleSections:secs.map(function(s){var h=s.querySelector('h3,h4');return h?h.textContent.trim():'';}),
    focusRows:document.querySelectorAll('.row-focus,.node-focus').length,
    tableRows:document.querySelectorAll('.el-table__body tbody tr').length,
    docOverflowX:document.documentElement.scrollWidth-document.documentElement.clientWidth});
})()