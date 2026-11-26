(function(){
  var re=/[0-9]+\.[0-9]{6,}/g;var out=[];
  var nodes=document.querySelectorAll('body *');
  for(var i=0;i<nodes.length;i++){var e=nodes[i];
    // 只看元素自身的直接文本节点，避免父级重复
    var own='';for(var c=0;c<e.childNodes.length;c++){if(e.childNodes[c].nodeType===3)own+=e.childNodes[c].textContent}
    var m=own.match(re);if(m){var r=e.getBoundingClientRect();var cs=getComputedStyle(e);
      out.push({tag:e.tagName,cls:String(e.className||'').slice(0,40),txt:own.trim().slice(0,60),
        w:Math.round(r.width),h:Math.round(r.height),disp:cs.display,vis:cs.visibility,op:cs.opacity,
        parent:String(e.parentElement&&e.parentElement.className||'').slice(0,30)});
    }
    if(out.length>=15)break;
  }
  return JSON.stringify({vw:window.innerWidth,vh:window.innerHeight,found:out});
})();
