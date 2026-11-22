(function(){var out=[];var els=[].slice.call(document.querySelectorAll('*'));
els.forEach(function(e){var t=(e.childElementCount===0?(e.innerText||'').trim():'');
if(t&&/开始|第 1 关|挑战|进入/.test(t)&&t.length<40){out.push({tag:e.tagName,cls:String(e.className||'').slice(0,80),txt:t})}});
return JSON.stringify(out.slice(0,20));})()