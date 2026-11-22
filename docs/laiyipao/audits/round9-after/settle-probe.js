(function(){var q=function(s){return document.querySelector(s)};var t=document.body.innerText||'';
return JSON.stringify({nextLevel:q('.next-level-btn')?(q('.next-level-btn').innerText||'').trim():null,
hasSettle:t.indexOf('通关')>=0||t.indexOf('结算')>=0||t.indexOf('得分')>=0,
settleFeedback:t.indexOf('反应伤害占比')>=0||t.indexOf('取卡')>=0,
settleAttr:q('.settle-attribution')?(q('.settle-attribution').innerText||'').trim().slice(0,200):null,
excerpt:t.slice(0,260)});})()