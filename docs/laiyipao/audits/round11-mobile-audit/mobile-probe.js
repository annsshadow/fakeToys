(function(){
  var vw=window.innerWidth,vh=window.innerHeight;
  var doc=document.documentElement;
  function vis(el){var c=getComputedStyle(el);return c.display!=='none'&&c.visibility!=='hidden'&&c.opacity!=='0'}
  function txt(el){return (el.innerText||el.textContent||'').trim()}
  // ① 文字竖排/异常换行：窄宽(<34px)但多字(>=4)且高>宽*2.2
  var vertical=[];
  var nodes=document.querySelectorAll('uni-text,text,div,span,p,h1,h2,h3,h4,button,.uni-button');
  for(var i=0;i<nodes.length;i++){var el=nodes[i];if(!vis(el))continue;var s=txt(el);if(s.length<4)continue;
    var r=el.getBoundingClientRect();if(r.width<=0)continue;
    if(r.width<34 && r.height>r.width*2.2 && s.length>=4){vertical.push({t:s.slice(0,18),n:s.length,w:Math.round(r.width),h:Math.round(r.height)})}
  }
  // ② 水平溢出
  var hOverflow=doc.scrollWidth-doc.clientWidth;
  var overflowEls=[];
  var all2=document.querySelectorAll('body *');
  for(var j=0;j<all2.length;j++){var e=all2[j];if(!vis(e))continue;var rr=e.getBoundingClientRect();
    if(rr.width>0 && rr.right>vw+2 && rr.left<vw){overflowEls.push({tag:e.tagName,cls:(e.className&&String(e.className).slice(0,30))||'',right:Math.round(rr.right)})}
    if(overflowEls.length>=6)break;
  }
  // ③ 关键按钮/胶囊超出视口底部或右侧
  var outView=[];
  var btns=document.querySelectorAll('button,.uni-button,[role=button],.btn,.btn-primary,.hud-pill,.star-gap-text,.cta');
  for(var k=0;k<btns.length;k++){var b=btns[k];if(!vis(b))continue;var br=b.getBoundingClientRect();if(br.width<=0)continue;
    if(br.right>vw+2||br.bottom>vh+2||br.left< -2){outView.push({t:txt(b).slice(0,14)||b.className,right:Math.round(br.right),bottom:Math.round(br.bottom),left:Math.round(br.left)})}
    if(outView.length>=8)break;
  }
  // ④ 固定定位元素覆盖内容
  var fixedEls=[];
  var all4=document.querySelectorAll('body *');
  for(var m=0;m<all4.length;m++){var f=all4[m];if(!vis(f))continue;var cs=getComputedStyle(f);
    if(cs.position==='fixed'){var fr=f.getBoundingClientRect();if(fr.width>0&&fr.height>0)fixedEls.push({cls:String(f.className||'').slice(0,28),z:cs.zIndex||'auto',top:Math.round(fr.top),h:Math.round(fr.height)})}
    if(fixedEls.length>=8)break;
  }
  // ⑤ 空态
  var bodyText=txt(document.body);
  var hasEmpty=/暂无|还没有|空空|无数据|没有/.test(bodyText);
  var p=window.__LYP_PROBE__||{};
  return JSON.stringify({
    url:location.href,title:document.title,vw:vw,vh:vh,
    textLen:bodyText.length,hasToken:(localStorage.getItem('lyp_token')||'').length>0,
    vertical:vertical.slice(0,6),hOverflow:hOverflow,overflowEls:overflowEls,
    outView:outView,fixedEls:fixedEls,empty:hasEmpty,
    scrollH:doc.scrollHeight,
    probe:{pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}
  })
})();