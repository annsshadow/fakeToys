(function(){
  var de=document.documentElement;
  var sk=document.querySelectorAll(".page-skeleton");
  var skVisible=[].slice.call(sk).filter(function(e){var s=getComputedStyle(e);return s.display!=="none"&&e.getBoundingClientRect().height>0;});
  var maskAll=document.querySelectorAll(".el-loading-mask");
  var gridMask=document.querySelectorAll(".data-grid .el-loading-mask");
  var statMask=document.querySelectorAll(".stat-row .el-loading-mask");
  var kpiCards=document.querySelectorAll(".page-skeleton .sk-kpis .sk-card");
  var skBars=document.querySelectorAll(".page-skeleton .skeleton-bar");
  var skRows=document.querySelectorAll(".page-skeleton .skeleton-row");
  var kpiEls=[].slice.call(document.querySelectorAll(".kpi-strip .kpi-value"));
  var kpiVals=kpiEls.map(function(e){return (e.textContent||"").trim();});
  var rows=document.querySelectorAll(".data-grid .el-table__row");
  var rc=document.querySelector(".result-count");
  var title=document.querySelector(".page-title");
  var role=sk.length?sk[0].getAttribute("role"):null;
  var busy=sk.length?sk[0].getAttribute("aria-busy"):null;
  function vis(e){if(!e)return false;var s=getComputedStyle(e);return s.display!=="none"&&e.getBoundingClientRect().height>0;}
  var o={
    href:location.href, innerW:window.innerWidth, scrollW:de.scrollWidth, clientW:de.clientWidth,
    hOverflow:de.scrollWidth>window.innerWidth+1,
    skeletonCount:sk.length, skeletonVisible:skVisible.length,
    skRole:role, skAriaBusy:busy,
    skKpiCards:kpiCards.length, skBars:skBars.length, skRows:skRows.length,
    maskCount:maskAll.length, gridMaskCount:gridMask.length, statMaskCount:statMask.length,
    maskVisible:[].slice.call(maskAll).filter(vis).length,
    kpiValues:kpiVals, tableRows:rows.length,
    resultCount:rc?(rc.textContent||"").trim():null,
    skVisibleKpi:kpiCards.length>0&&[].slice.call(kpiCards).every(vis),
    title:title?(title.textContent||"").trim():null,
    isLogin:location.hash.indexOf("/admin/login")>=0||!!document.querySelector(".login-page"),
    bodyLen:(document.body.innerText||"").length,
    hits:(window.__qaHits||[]).map(function(h){return h.u;}),
    ready:!!window.__R93_READY
  };
  console.log("PROBE_RESULT "+JSON.stringify(o)); return o;
})();
