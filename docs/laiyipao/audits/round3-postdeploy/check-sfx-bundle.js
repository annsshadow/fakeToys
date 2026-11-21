(async function(){
  try {
    var scripts = [].slice.call(document.querySelectorAll('script[src]')).map(function(s){return s.src;});
    var out = {scripts:scripts, found:false, match:''};
    for (var i=0;i<scripts.length;i++) {
      try {
        var r = await fetch(scripts[i]); var txt = await r.text();
        if (txt.indexOf('play() request was interrupted') >= 0) { out.found=true; out.match=scripts[i].split('/').pop(); break; }
      } catch(e){}
    }
    return JSON.stringify(out);
  } catch(e){ return 'ERR '+e.message; }
})()