(function(){
  window.__F2_DIAG__ = {interrupted:0, preventedTrue:0, preventedFalse:0};
  // 后置监听器（sfx 过滤器之后注册）；若过滤器已 preventDefault，这里读到的 defaultPrevented=true
  window.addEventListener('unhandledrejection', function(ev){
    var msg = String(ev.reason && ev.reason.message || ev.reason || '');
    if (msg.indexOf('play() request was interrupted') >= 0) {
      window.__F2_DIAG__.interrupted++;
      if (ev.defaultPrevented) window.__F2_DIAG__.preventedTrue++;
      else window.__F2_DIAG__.preventedFalse++;
    }
  });
  return 'listener installed';
})()