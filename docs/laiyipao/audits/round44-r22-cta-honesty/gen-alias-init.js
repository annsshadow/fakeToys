#!/usr/bin/env node
// 从 stdin 读取 guest 登录响应，生成浏览器 init-script；不回显任何凭证真值。
const fs = require('node:fs')
const out = process.argv[2]
if (!out) { console.error('MISSING_OUT'); process.exit(1) }
let raw = ''
process.stdin.on('data', (c) => { raw += c })
process.stdin.on('end', () => {
  let d
  try { d = JSON.parse(raw) } catch { console.error('BAD_RESPONSE'); process.exit(1) }
  // /auth/guest 的 user 摘要不带 max_stage；解锁状态由 shell 从 QA 库单独断言。
  if (!d.access_token || !d.user || d.user.id !== 10) {
    console.error('BAD_USER_TOKEN'); process.exit(1)
  }
  const exp = d.expires_at || new Date(Date.now() + 3600e3).toISOString()
  const profile = JSON.stringify({ nickname: d.user.nickname || '玩家', isGuest: true })
  const lines = [
    "try { localStorage.setItem('lyp_token', " + JSON.stringify(d.access_token) + "); } catch (e) {}",
    "try { localStorage.setItem('lyp_refresh', " + JSON.stringify(d.refresh_token || '') + "); } catch (e) {}",
    "try { localStorage.setItem('lyp_token_exp', " + JSON.stringify(exp) + "); } catch (e) {}",
    "try { localStorage.setItem('lyp_guest', " + JSON.stringify(d.guest_token || '') + "); } catch (e) {}",
    "try { localStorage.setItem('lyp_profile', " + JSON.stringify(profile) + "); } catch (e) {}",
    "try { localStorage.setItem('lyp_battle_speed', '8'); } catch (e) {}",
  ].join('\n') + '\n'
  fs.writeFileSync(out, lines, { mode: 0o600 })
  console.log('AUTH_INIT_WRITTEN user=10')
})
