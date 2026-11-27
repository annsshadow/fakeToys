(function(){
  var out={};
  var btn=[].slice.call(document.querySelectorAll('.btn')).find(function(b){return (b.textContent||'').indexOf('取复现')>=0});
  out.found=!!btn;
  if(!btn) return JSON.stringify(out);
  var node=btn;
  var depth=0;
  var insts=[];
  while(node && depth<12){
    var c=node.__vueParentComponent;
    if(c && c.setupState){
      var keys=Object.keys(c.setupState).filter(function(k){return /fetch|replay|falsif|submit|fetching/i.test(k)});
      insts.push({tag:node.tagName||node.nodeName,keys:keys,depth:depth});
    }
    node=node.parentElement; depth++;
  }
  out.insts=insts;
  var cs=getComputedStyle(btn);
  out.btnStyle={pointerEvents:cs.pointerEvents,cursor:cs.cursor,opacity:cs.opacity,tabIndex:btn.tabIndex};
  return JSON.stringify(out);
})()
