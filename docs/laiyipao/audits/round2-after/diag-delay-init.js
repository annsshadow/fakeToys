(function(){
  window.__DELAY_DIAG__={sendCalled:0,delayed:0,urls:[]};
  var oo=XMLHttpRequest.prototype.open;
  XMLHttpRequest.prototype.open=function(m,u){this.__u=u;return oo.apply(this,arguments);};
  var os=XMLHttpRequest.prototype.send;
  XMLHttpRequest.prototype.send=function(){
    window.__DELAY_DIAG__.sendCalled++;
    var u=String(this.__u||'');
    window.__DELAY_DIAG__.urls.push(u);
    if(u.indexOf('/announcements')>=0){
      window.__DELAY_DIAG__.delayed++;
      var self=this,args=arguments;
      setTimeout(function(){try{os.apply(self,args);}catch(e){window.__DELAY_DIAG__.err=String(e)}},5000);
      return;
    }
    return os.apply(this,arguments);
  };
})();