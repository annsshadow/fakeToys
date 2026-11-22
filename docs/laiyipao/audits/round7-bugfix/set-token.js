// 仅作占位模板：真实 token 禁止写死入库（AGENTS.md 零真值硬门槛）。
// 复现时从 POST /auth/guest 响应 access_token 动态获取并替换 <YOUR_TEST_TOKEN>，
// 或使用同目录 set-token.js.tpl 的 __TOKEN__ 模板在未提交目录本地实例化。
(function(){localStorage.setItem('lyp_token','<YOUR_TEST_TOKEN>');return 'SETLEN '+localStorage.getItem('lyp_token').length})()
