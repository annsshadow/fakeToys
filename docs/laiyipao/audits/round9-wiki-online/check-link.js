(function(){
  var a=document.querySelector('.wiki-link');
  return a?('LINK_YES href='+(a.getAttribute('href')||'')):'LINK_NO';
})()