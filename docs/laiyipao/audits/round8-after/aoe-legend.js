(function(){var qa=function(s){return [].slice.call(document.querySelectorAll(s))};var all=qa('*');
var legend=all.filter(function(e){var a=(e.getAttribute&&e.getAttribute('aria-label'))||'';return a.indexOf('图例')>=0});
if(legend.length)legend[legend.length-1].click();
var t=document.body.innerText||'';
var pct=(t.match(/25\s*%/g)||[]).length;var rad=(t.match(/120/g)||[]).length;
return JSON.stringify({legendBtns:legend.length,click:legend.length?'clicked':'none',pct25:pct,radius120:rad,
hasSteam:t.indexOf('蒸汽')>=0,hasOverheat:t.indexOf('过热')>=0,excerpt:t.slice(0,200)});})()