(async function(){
  try {
    var s = document.querySelector('script[src]');
    var txt = await (await fetch(s.src)).text();
    var out = {bundleBytes:txt.length, interrupted:txt.split('interrupted').length-1, preventDefault:txt.split('preventDefault').length-1, unhandledrejection:txt.split('unhandledrejection').length-1, playRequest:txt.split('play() request').length-1};
    // 提取 interrupted 附近上下文
    var idx = txt.indexOf('interrupted');
    out.ctx = idx>=0 ? txt.slice(Math.max(0,idx-60), idx+60) : '';
    return JSON.stringify(out);
  } catch(e){ return 'ERR '+e.message; }
})()