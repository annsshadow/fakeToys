(function(){
  var Native = window.XMLHttpRequest;
  window.__raceCtl = { delayMatch: "", delayMs: 0, remaining: 0, log: [] };
  function Patched(){
    var x = new Native();
    var _open = x.open;
    x.open = function(m,u){ try{ x.__u = String(u || ""); }catch(e){ x.__u = ""; } return _open.apply(x, arguments); };
    var _send = x.send;
    x.send = function(){
      var ctl = window.__raceCtl, u = x.__u || "";
      try{ ctl.log.push({p:"send",u:u,t:Date.now()}); }catch(e){}
      if(ctl.delayMatch && u.indexOf(ctl.delayMatch) >= 0 && ctl.remaining > 0){
        ctl.remaining--; var self=x, args=arguments;
        setTimeout(function(){ _send.apply(self,args); }, ctl.delayMs);
        return;
      }
      return _send.apply(x, arguments);
    };
    try{ x.addEventListener("loadend", function(){ try{ window.__raceCtl.log.push({p:"done",u:x.__u||"",t:Date.now()}); }catch(e){} }); }catch(e){}
    return x;
  }
  Patched.prototype = Native.prototype;
  window.XMLHttpRequest = Patched;
})();
