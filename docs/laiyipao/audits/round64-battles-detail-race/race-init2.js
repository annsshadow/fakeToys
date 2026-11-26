(function(){var N=window.XMLHttpRequest;window.__raceCtl={rules:[],log:[]};
function P(){var x=new N(),o=x.open,s=x.send;
x.open=function(m,u){try{x.__u=String(u||"");}catch(e){x.__u="";}return o.apply(x,arguments);};
x.send=function(){var c=window.__raceCtl,u=x.__u||"";try{c.log.push({p:"send",u:u,t:Date.now()});}catch(e){}
for(var i=0;i<c.rules.length;i++){var r=c.rules[i];if(r.match&&u.indexOf(r.match)>=0&&r.remaining>0){r.remaining--;var self=x,args=arguments;setTimeout(function(){s.apply(self,args);},r.ms);return;}}
return s.apply(x,arguments);};
try{x.addEventListener("loadend",function(){try{window.__raceCtl.log.push({p:"done",u:x.__u||"",t:Date.now()});}catch(e){}});}catch(e){}return x;}
P.prototype=N.prototype;window.XMLHttpRequest=P;})()
