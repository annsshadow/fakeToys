(function(){var target="__TARGET__";
var els=[].slice.call(document.querySelectorAll('view,div,text,button,span'));
var hit=els.filter(function(e){var t=(e.innerText||'').trim();return t===target||t.indexOf(target)>=0&&t.length<target.length+18}).pop();
if(!hit)return 'NOT_FOUND:'+target; hit.click(); return 'CLICKED:'+target+' :: '+(hit.className||'')})()