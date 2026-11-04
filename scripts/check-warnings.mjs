#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// 全仓「编译告警零容忍」一键核验。
//
// 用法：
//   node scripts/check-warnings.mjs              # 跑全部子项目
//   node scripts/check-warnings.mjs --list       # 只列出检查项
//   node scripts/check-warnings.mjs --only rust,admin
//   node scripts/check-warnings.mjs --verbose    # 打印每条命中行
//
// 判据是**编译器自己的输出**，不是"命令退出码为 0"：很多工具链的告警不改变
// 退出码（Vite / Rollup / lightningcss / Python 的 SyntaxWarning 都是这样），
// 所以每项检查除了要求 exit 0，还要对输出做正则匹配，命中即失败。
//
// 各子项目自身也已经把「告警=错误」写进配置，本脚本是那套门禁的统一入口：
//   - Rust  : oa4rust/backend/Cargo.toml 的 [workspace.lints.rust] warnings = "deny"
//   - Python: -W error::SyntaxWarning
//   - Go    : go vet（发现即非零退出）+ gofmt -l
//   - TS/Vue: tsc/vue-tsc 无告警概念，故由本脚本匹配构建输出

import { spawn, spawnSync } from 'node:child_process'
import { existsSync, readdirSync, readFileSync } from 'node:fs'
import { homedir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..')

// cargo 常装在用户目录下；GUI / 精简 PATH 环境下不会被继承。
const extraPath = join(homedir(), '.cargo', 'bin')
const env = {
  ...process.env,
  PATH: existsSync(extraPath) ? `${extraPath}${process.platform === 'win32' ? ';' : ':'}${process.env.PATH}` : process.env.PATH,
}

/** 匹配「告警行」的正则。命中行数 > 0 即判该项失败。 */
const PATTERNS = {
  // --message-format=short 的行形如 `crates/x/src/y.rs:12:5: warning: unused ...`
  // （告警不以行首开头）；清单里 deny(warnings) 之后这些会变成 error:，退出码也会非零。
  rust: /: warning: |: error(\[|: )/,
  vite: /^[(]!|\[lightningcss|(?:^|\s)warning:|is not recognized as a valid pseudo-class/i,
  tsc: /error TS\d+|warning TS\d+/i,
  gofmt: /^[^\s].*\.go$/,
  python: /SyntaxWarning|SyntaxError|DeprecationWarning/i,
}

// Vite/Rollup 的告警行带 ANSI 颜色码（"\033[33m(!) ..."），不剥掉的话 ^(!) 一条都匹配不到。
const ANSI = /\u001b\[[0-9;]*m/g

/**
 * 静态守卫：每个 workspace 成员都必须显式继承 `[lints] workspace = true`。
 *
 * `[workspace.lints]` 不会自动作用到成员 crate —— 成员不 opt-in 就**静默失去**
 * deny(warnings)。新加一个 crate 时最容易漏这一步，而且漏了没有任何症状，
 * 所以单独设一条检查。
 */
function checkLintsCoverage() {
  const backend = join(ROOT, 'oa4rust', 'backend')
  const cratesDir = join(backend, 'crates')
  const manifests = [
    join(backend, 'Cargo.toml'),
    ...readdirSync(cratesDir, { withFileTypes: true })
      .filter((d) => d.isDirectory())
      .map((d) => join(cratesDir, d.name, 'Cargo.toml'))
      .filter(existsSync),
  ]
  const missing = manifests.filter((m) => !/\[lints\][^[]*workspace\s*=\s*true/.test(readFileSync(m, 'utf8')))
  return { ok: missing.length === 0, hits: missing.map((m) => `${m.slice(ROOT.length + 1)} 缺少 [lints] workspace = true`) }
}

/**
 * 静态守卫：admin 前端里的「服务端枚举副本」不能与服务端各自漂移。
 *
 * 已发生的两处分叉（第 55 轮）：
 *  - 商城价格币种：服务端可作价集合 = `walletColumns` + `walletTokens`（7 个），
 *    而 `EconomyView.vue` 的 `CURRENCY_LABEL` 只抄了货币列那 4 个；
 *    `parsePriceMap` 因此拒绝 `{"mastery_reset":5}` 这类服务端合法定价。
 *  - 审计动作中文标签：服务端新增 action（update_shop_item / verify_replay …）
 *    后 admin 的 `ACTION_LABEL` 没跟上，审计页只能显示英文机器值。
 *
 * 判据：货币集合**双向**相等（多一个少一个都失败）；审计 action 只要求
 * 「服务端出现的每个 action 都有标签」——标签侧保留历史别名是允许的，
 * 而漏掉新 action 正是要防的那类静默缺口。
 */
function checkAdminConfigDrift() {
  const hits = []
  const read = (rel) => readFileSync(join(ROOT, rel), 'utf8')

  const serviceGo = read('laiyipao/server/internal/service/service.go')
  const mapKeys = (src, re) => {
    const m = src.match(re)
    return m ? [...m[1].matchAll(/"([a-z_]+)"\s*:/g)].map((x) => x[1]) : []
  }
  const serverCurrencies = new Set([
    ...mapKeys(serviceGo, /var walletColumns = map\[string\]string\{([\s\S]*?)\}/),
    ...mapKeys(serviceGo, /var walletTokens = map\[string\]bool\{([\s\S]*?)\}/),
  ])

  const economyVue = read('laiyipao/admin/src/views/EconomyView.vue')
  const cm = economyVue.match(/const CURRENCY_LABEL: Record<string, string> = \{([\s\S]*?)\n\}/)
  const adminCurrencies = new Set(cm ? [...cm[1].matchAll(/^\s*([a-z_]+):/gm)].map((x) => x[1]) : [])

  for (const k of serverCurrencies) if (!adminCurrencies.has(k)) hits.push(`CURRENCY_LABEL 缺服务端可作价币种 ${k}`)
  for (const k of adminCurrencies) if (!serverCurrencies.has(k)) hits.push(`CURRENCY_LABEL 多了服务端不认的币种 ${k}`)

  // 审计 action 字面量散在三种写法里：后台事务内 AuditTx、handler 里的 Svc.Audit、
  // 玩家侧裸 INSERT，以及 stats.go 按分支赋值的 action 变量。
  const serverActions = new Set()
  const serviceDir = join(ROOT, 'laiyipao/server/internal/service')
  for (const f of readdirSync(serviceDir)) {
    if (!f.endsWith('.go') || f.endsWith('_test.go')) continue
    const src = readFileSync(join(serviceDir, f), 'utf8')
    for (const re of [
      /AuditTx\(\s*ctx,\s*tx,\s*audit\[0\],\s*"([a-z_]+)"/g,
      /action\s*(?::=|=)\s*"([a-z_]+)"/g,
      /"user#%d",\s*"([a-z_]+)"/g,
    ]) {
      for (const x of src.matchAll(re)) serverActions.add(x[1])
    }
  }
  const handlersAdmin = read('laiyipao/server/internal/httpapi/handlers_admin.go')
  for (const x of handlersAdmin.matchAll(/\.Audit\(\s*(?:ctx|c\.Context\(\)),\s*adminIDFrom\(c\),\s*"([a-z_]+)"/g)) serverActions.add(x[1])

  const contentVue = read('laiyipao/admin/src/views/ContentView.vue')
  const am = contentVue.match(/const ACTION_LABEL: Record<string, string> = \{([\s\S]*?)\n\}/)
  const adminActions = new Set(am ? [...am[1].matchAll(/^\s*([a-z_]+):/gm)].map((x) => x[1]) : [])
  for (const a of [...serverActions].sort()) {
    if (!adminActions.has(a)) hits.push(`ACTION_LABEL 缺服务端审计 action ${a}`)
  }

  // 解析失效（源文件结构变了）不能被当成「两边都空 = 一致」放行。
  if (serverCurrencies.size === 0 || adminCurrencies.size === 0 || serverActions.size === 0 || adminActions.size === 0) {
    hits.push('守卫自身没解析到集合（模式匹配已失效），先修守卫再放行')
  }
  return { ok: hits.length === 0, hits }
}

/**
 * 静态守卫：每一项命令里引用的本地脚本，必须**存在且被 git 跟踪**。
 *
 * O13 的空转根因：`oa4rust/backend/scripts/check_openapi_up_to_date.py` 从未入库
 * （整个 scripts 目录被 .gitignore 忽略），本地写好了能跑，任何干净克隆 / CI
 * 上都只有 exit=2。这条守卫把同类问题变成一句可读的 FAIL，而不是等某个
 * 检查项跑到一半报「找不到文件」。
 */
function checkScriptPaths() {
  const hits = []
  const seen = new Set()
  for (const c of CHECKS) {
    if (typeof c.cmd !== 'string') continue
    const base = join(ROOT, c.cwd)
    for (const m of c.cmd.matchAll(/(?:^|[\s;&|])(scripts\/[^\s;&|"']+)/g)) {
      const rel = m[1]
      const key = `${c.cwd}\u0000${rel}`
      if (seen.has(key)) continue
      seen.add(key)
      if (!existsSync(join(base, rel))) {
        hits.push(`${c.id}: 引用的脚本不存在 ${c.cwd}/${rel}`)
        continue
      }
      const tracked = spawnSync('git', ['ls-files', '--error-unmatch', join(c.cwd, rel)], {
        cwd: ROOT,
        env,
        encoding: 'utf8',
      })
      if (tracked.status !== 0) {
        hits.push(`${c.id}: 引用的脚本未被 git 跟踪 ${c.cwd}/${rel}（干净克隆 / CI 上必然空转）`)
      }
    }
  }
  return { ok: hits.length === 0, hits }
}

// crate 数量动态数：曾经 label 里硬编码「97 crates」，crate 增删后静默漂移
// （实测 96 个目录 + 根包）。label 只该描述现状，不该是一个要人维护的常数。
const RUST_CRATE_COUNT =
  1 + readdirSync(join(ROOT, 'oa4rust', 'backend', 'crates'), { withFileTypes: true })
    .filter((d) => d.isDirectory()).length

/** 某 Python 模块在当前 `python` 下是否可导入（用于 requires 守门）。 */
function hasPythonModule(module) {
  // 不能加 `shell: true`：Node 会把 args 直接空格拼接，`python -c import pytest`
  // 丢掉引号后 Python 只收到 `import`（SyntaxError），探针永远返回 false，
  // 于是 llm-lab-tests 在 CI 里一直静默 SKIP（2026-10-28 实测）。
  const run = spawnSync('python', ['-c', `import ${module}`], { env, encoding: 'utf8' })
  return run.status === 0
}

const CHECKS = [
  {
    id: 'rust-lints',
    label: 'oa4rust/backend 全部成员继承 deny(warnings)',
    custom: checkLintsCoverage,
  },
  {
    id: 'admin-config-drift',
    label: 'laiyipao admin 枚举映射与服务端白名单一致',
    custom: checkAdminConfigDrift,
  },
  {
    id: 'script-paths',
    label: '各项引用的本地脚本存在且已入库（防空转）',
    custom: checkScriptPaths,
  },
  {
    id: 'rust',
    label: `oa4rust/backend (Rust, ${RUST_CRATE_COUNT} crates)`,
    cwd: 'oa4rust/backend',
    cmd: 'cargo clippy --workspace --all-targets --message-format=short',
    pattern: PATTERNS.rust,
    note: 'clippy 是 cargo check 的超集；清单里已 deny(warnings)',
    timeout: 30 * 60 * 1000,
  },
  {
    id: 'desktop',
    label: 'oa4rust desktop (Vue + Vite)',
    cwd: 'oa4rust',
    cmd: 'pnpm build',
    pattern: PATTERNS.vite,
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'mobile',
    label: 'oa4rust mobile (uni-app H5 + 微信小程序)',
    cwd: 'oa4rust',
    cmd: 'pnpm --filter @oa4rust/mobile build:h5 && pnpm --filter @oa4rust/mobile build:mp-weixin',
    pattern: PATTERNS.vite,
    timeout: 15 * 60 * 1000,
  },
  {
    id: 'go',
    label: 'laiyipao/server (Go)',
    cwd: 'laiyipao/server',
    cmd: 'go build ./... && go vet ./... && gofmt -l .',
    pattern: PATTERNS.gofmt,
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'admin',
    label: 'laiyipao/admin (Vue + Vite)',
    cwd: 'laiyipao/admin',
    cmd: 'npm run build',
    pattern: PATTERNS.vite,
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'miniapp',
    label: 'laiyipao/miniapp (uni-app H5)',
    cwd: 'laiyipao/miniapp',
    cmd: 'npm run build:h5',
    pattern: PATTERNS.vite,
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'auto-checkin',
    label: 'auto-checkin (TypeScript, typecheck + 测试)',
    cwd: 'auto-checkin',
    // 2026-10-24 补上 npm test：此前只有 tsc，而「口令模式下面板不可用」
    // 那类 bug 恰好只在行为面暴露（静态路径 401），类型面全绿也抓不到。
    cmd: 'npx tsc --noEmit && npm test',
    pattern: PATTERNS.tsc,
    timeout: 5 * 60 * 1000,
  },
  {
    id: 'augmentor-web',
    label: 'augmentor/web (React + Vite)',
    cwd: 'augmentor/web',
    cmd: 'npm run build && npm run lint',
    pattern: PATTERNS.vite,
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'augmentor',
    label: 'augmentor (Python 字节码编译)',
    cwd: 'augmentor',
    cmd: 'python -W error::SyntaxWarning -m compileall -q -f -x "(node_modules|__pycache__|\\.venv|Temp|bak|data)" augmentor api cli.py',
    pattern: PATTERNS.python,
    timeout: 5 * 60 * 1000,
  },
  {
    id: 'llm-lab',
    label: 'llm-lab (Python 字节码编译)',
    cwd: 'llm-lab',
    cmd: 'python -W error::SyntaxWarning -m compileall -q -f -x "(node_modules|__pycache__|\\.venv)" .',
    pattern: PATTERNS.python,
    timeout: 5 * 60 * 1000,
  },
  {
    id: 'llm-lab-tests',
    label: 'llm-lab (pytest 行为面)',
    cwd: 'llm-lab',
    cmd: 'python -m pytest -q --no-header -p no:cacheprovider',
    pattern: PATTERNS.python,
    // requires 守门：解释器没装 pytest 时 SKIP 并说明，
    // 不像命令失败那样制造环境性红。本机 managed python 无 pytest；
    // 装了的开发机（或 CI 的 python）会自动接上这一项。
    requires: () => (hasPythonModule('pytest') ? true : '当前 python 无 pytest 模块'),
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'r7-readonly-cross-check',
    label: 'oa4rust R7 只读挂写交叉验证（0 疑似含写）',
    cwd: 'oa4rust/backend',
    cmd: 'python scripts/cross_check_r7_readonly.py',
    // 周期回归检查（2026-10-30）：对尺子判为「R7 只读挂写方法」的全部路由用独立
    // 判据（注释剥离 + 写 SQL 关键字 + ORM 写扫描）复核，出现疑似含写候选即失败。
    // 与 audit_write_gating.mjs 的 writeEvidence 是不同实现，互为印证。
    pattern: /疑似含写候选/,
    timeout: 2 * 60 * 1000,
  },
  {
    id: 'license',
    label: 'license header（全仓源码 SPDX 标识符）',
    cwd: '.',
    cmd: 'node scripts/check-license-headers.mjs',
    // 第 63 轮：git 跟踪的 ts/tsx/js/mjs/cjs/go/rs/py 源码缺 SPDX 标识符即失败。
    // 脚本非零退出即 FAIL；pattern 只匹配失败时的「缺 header：xxx」详情行
    // （成功时输出的是「缺失 0」，不能被 pattern 命中误判 FAIL）。
    pattern: /缺 header：/,
    timeout: 2 * 60 * 1000,
  },
  {
    id: 'openapi-paths',
    label: 'oa4rust OpenAPI 生成物与路由源码不脱节（regen diff 为空）',
    cwd: 'oa4rust/backend',
    cmd: 'python scripts/check_openapi_up_to_date.py',
    // 第 68 轮：跑 gen_openapi_paths.py 后与已提交产物逐字比对（行尾无关）。
    // 脚本总是恢复原产物内容（检查本身不改仓库）；非零退出即 FAIL。
    // pattern 只匹配失败时的「落后于路由源码」行。
    pattern: /落后于路由源码|gen_openapi_paths\.py 执行失败/,
    timeout: 2 * 60 * 1000,
  },
  {
    id: 'generated-tests',
    label: 'oa4rust tests_generated.rs 与生成器逐字一致（O14 门禁）',
    cwd: 'oa4rust/backend',
    cmd: 'python scripts/check_generated_tests_up_to_date.py',
    // O14：生成器跑到临时目录，与已提交的 91 个产物逐字节比对；不一致 exit 1。
    // 同时校验生成器被 git 跟踪、lib.rs 有 mod tests_generated;、集合规模下限。
    // pattern 只匹配失败详情行（成功时输出「OK:」，不能被命中）。
    pattern: /与生成器不一致|低于下限|未被 git 跟踪|缺少 `mod tests_generated;`|执行失败/,
    timeout: 5 * 60 * 1000,
  },
  {
    id: 'generated-artifacts',
    label: 'oa4rust 其余 Python 生成器产物与源码一致（MCP 路由表）',
    cwd: 'oa4rust/backend',
    cmd: 'python scripts/check_generated_artifacts_up_to_date.py',
    // O14 审计补上的第三类生成物：gen_mcp_tools.py → mcp_server/generated_routes.rs。
    // parity 的生成器因输入 JSON 已被删除无法真实判定，登记为债务（见台账）。
    pattern: /生成物与生成器不一致|执行失败|生成物缺失|生成器不存在/,
    timeout: 2 * 60 * 1000,
  },
]

const args = process.argv.slice(2)
const verbose = args.includes('--verbose')
if (args.includes('--list')) {
  for (const c of CHECKS) console.log(`${c.id.padEnd(14)} ${c.label}`)
  process.exit(0)
}
const onlyArg = args.find((a) => a.startsWith('--only'))
let only = null
if (onlyArg) {
  // 第 62 轮：`--only x` 缺下一参数（`--only` 落在末尾）时，
  // 之前 `args[idx+1]` 为 undefined 直接 `.split` 崩——给出人话报错。
  const raw = onlyArg.includes('=')
    ? onlyArg.split('=').slice(1).join('=')
    : args[args.indexOf(onlyArg) + 1]
  if (raw === undefined || raw === '') {
    console.error(`--only 缺少检查项（用法：--only id 或 --only=id,other）`)
    process.exit(2)
  }
  only = raw.split(',').map((s) => s.trim()).filter((s) => s.length > 0)
}

// 第 62 轮：`--only` 传入未知检查项（手误 typo）时，之前只落到
// 「没有匹配的检查项」——用户分不清是打错了还是空列表。这里把未知值点出来。
const validIds = new Set(CHECKS.map((c) => c.id))
const unknown = only ? only.filter((s) => !validIds.has(s)) : []
if (unknown.length > 0) {
  console.error(`未知检查项：${unknown.join(', ')}\n可用：${[...validIds].join(', ')}（或 --list 查看全部）`)
  process.exit(2)
}

const selected = only ? CHECKS.filter((c) => only.includes(c.id)) : CHECKS
if (selected.length === 0) {
  console.error(`没有匹配的检查项。可用：${CHECKS.map((c) => c.id).join(', ')}`)
  process.exit(2)
}

// 第 97 轮：检查项彼此独立（不同子项目 / 不同目录），旧实现串行 `spawnSync`
// 让整条门禁的耗时等于各项之和（clippy / 两个前端构建 / pytest 加起来占绝大部分）。
// 改为异步 `spawn` + 固定并发（默认 4，可用 CHECK_CONCURRENCY 调小，例如
// 低配机器 / 需要串行基线时设 1）：墙钟时间约等于最慢一项，而不是全部相加。
// 结果按下标回填，输出顺序与汇总口径不变。
const CONCURRENCY = Math.max(1, Number(process.env.CHECK_CONCURRENCY) || 4)
const MAX_OUTPUT = 64 * 1024 * 1024

/** 跑一个命令型检查，返回与串行版本同形状的结果对象。 */
function runCommandCheck(check) {
  return new Promise((resolveCheck) => {
    const skipped = check.requires ? check.requires() : null
    if (typeof skipped === 'string') {
      console.log(`- ${check.label} … SKIP (${skipped})`)
      resolveCheck({ ...check, status: 'SKIP', detail: skipped })
      return
    }

    // 旧写法 `check.cmd(skipped.jar)` 在无 `requires` 的检查上是 `null.jar`，
    // 一旦再出现函数型 cmd 就会 TypeError；`skipped` 此处可能是 null/true。
    const cmd = typeof check.cmd === 'function' ? check.cmd(skipped?.jar) : check.cmd
    const started = Date.now()
    let output = ''
    let truncated = false
    let settled = false
    let timer = null

    const child = spawn(cmd, {
      cwd: join(ROOT, check.cwd),
      env,
      shell: true,
    })

    const collect = (chunk) => {
      if (output.length >= MAX_OUTPUT) {
        truncated = true
        return
      }
      output += chunk
      if (output.length > MAX_OUTPUT) {
        output = output.slice(0, MAX_OUTPUT)
        truncated = true
      }
    }
    child.stdout?.setEncoding('utf8')
    child.stderr?.setEncoding('utf8')
    child.stdout?.on('data', collect)
    child.stderr?.on('data', collect)

    const finish = (status, extra = {}) => {
      if (settled) return
      settled = true
      if (timer) clearTimeout(timer)
      const clean = output.replace(ANSI, '')
      const hits = clean.split(/\r?\n/).filter((line) => check.pattern.test(line))
      const seconds = ((Date.now() - started) / 1000).toFixed(1)
      const failed = status !== 0 || hits.length > 0
      const entry = { ...check, status: failed ? 'FAIL' : 'PASS', hits, exit: status, seconds, ...extra }
      console.log(
        failed
          ? `- ${check.label} … FAIL (exit=${status}, 告警 ${hits.length} 条, ${seconds}s)`
          : `- ${check.label} … PASS (${seconds}s)`,
      )
      if (verbose || failed) {
        for (const line of hits.slice(0, 40)) console.log(`    ${line.trim()}`)
        if (hits.length > 40) console.log(`    … 另有 ${hits.length - 40} 条`)
        if (failed && hits.length === 0) console.log(clean.split(/\r?\n/).slice(-25).join('\n'))
      }
      if (truncated) console.log(`    （输出超过 ${MAX_OUTPUT} 字节，已截断）`)
      resolveCheck(entry)
    }

    child.on('error', (err) => finish(-1, { spawnError: err.message }))
    child.on('close', (code, signal) => finish(code ?? (signal ? -1 : 0), signal ? { signal } : {}))
    if (check.timeout) {
      timer = setTimeout(() => {
        truncated = true
        child.kill()
      }, check.timeout)
    }
  })
}

const results = new Array(selected.length)
const jobs = []
selected.forEach((check, index) => {
  // 不跑命令的静态检查（例如「成员是否都继承了 workspace lints」）先同步跑完。
  if (check.custom) {
    process.stdout.write(`- ${check.label} … `)
    let outcome
    try {
      outcome = check.custom()
    } catch (err) {
      // 守卫自身解析失败（源文件改名 / 结构变化）不能被当成异常吞掉，
      // 也不能让整个门禁崩在栈里：按项 FAIL，并说明原因。
      outcome = { ok: false, hits: [`守卫自身执行异常：${err.message}`] }
    }
    results[index] = { ...check, status: outcome.ok ? 'PASS' : 'FAIL', hits: outcome.hits }
    console.log(outcome.ok ? 'PASS' : `FAIL (${outcome.hits.length} 处)`)
    if (!outcome.ok) for (const line of outcome.hits) console.log(`    ${line}`)
    return
  }
  // 注意这里只登记「怎么跑」，不调用 —— `runCommandCheck` 一被调用就会 spawn，
  // 提前调用等于无视并发上限、把全部检查一次性拉起来。
  jobs.push(async () => {
    const entry = await runCommandCheck(check)
    results[index] = entry
  })
})

// 固定并发池：任一 worker 空闲就领下一项，最多 CONCURRENCY 个同时在跑。
let nextJob = 0
const workers = Array.from({ length: Math.min(CONCURRENCY, jobs.length) }, async () => {
  while (nextJob < jobs.length) {
    await jobs[nextJob++]()
  }
})
await Promise.all(workers)

const failedCount = results.filter((r) => r.status === 'FAIL').length
const skippedCount = results.filter((r) => r.status === 'SKIP').length
console.log(`\n${results.length} 项检查：${results.length - failedCount - skippedCount} 通过 / ${failedCount} 失败 / ${skippedCount} 跳过`)
process.exit(failedCount === 0 ? 0 : 1)
