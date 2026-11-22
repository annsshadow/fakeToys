// 仅作占位模板：真实 token 禁止写死入库（AGENTS.md 零真值硬门槛）。
// 复现时从 POST /auth/guest 响应 access_token 动态获取并替换 <YOUR_TEST_TOKEN>。
// 同时清除 lyp_token_exp 残留，避免 api/client.ts hasUsableToken() 判旧过期后重新游客建号。
(function(){localStorage.setItem('lyp_token','<YOUR_TEST_TOKEN>');localStorage.removeItem('lyp_token_exp');return 'SETLEN '+localStorage.getItem('lyp_token').length+' EXP='+localStorage.getItem('lyp_token_exp')})()
