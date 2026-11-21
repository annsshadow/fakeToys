(function(){
  // 延迟 /announcements 请求 3.5s 发出，制造骨架屏 loading 窗口（uni.request H5 走 XHR）
  var origSend = XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.send = function(){
    var url = this.__lypUrl || '';
    if (String(url).indexOf('/announcements') >= 0) {
      var self = this, args = arguments;
      setTimeout(function(){ try { origSend.apply(self, args); } catch(_){} }, 3500);
      return;
    }
    return origSend.apply(this, arguments);
  };
  var origOpen = XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open = function(m, u){ this.__lypUrl = u; return origOpen.apply(this, arguments); };
})();