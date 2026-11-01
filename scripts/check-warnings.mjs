#!/usr/bin/env node
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
//   - Java  : cool/compile.bat 的 -Xlint:all -Werror
//   - Python: -W error::SyntaxWarning
//   - Go    : go vet（发现即非零退出）+ gofmt -l
//   - TS/Vue: tsc/vue-tsc 无告警概念，故由本脚本匹配构建输出

import { spawnSync } from 'node:child_process'
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
  java: /warning:|警告:/i,
}

// Vite/Rollup 的告警行带 ANSI 颜色码（"\033[33m(!) ..."），不剥掉的话 ^(!) 一条都匹配不到。
const ANSI = /\u001b\[[0-9;]*m/g

function findCommonsCodec() {
  const lib = join(ROOT, 'cool', 'lib')
  if (existsSync(lib)) {
    const hit = readdirSync(lib).find((f) => /^commons-codec.*\.jar$/i.test(f))
    if (hit) return join(lib, hit)
  }
  const fromEnv = process.env.COMMONS_CODEC_JAR
  if (fromEnv && existsSync(fromEnv)) return fromEnv
  // 本机 Maven 仓库兜底：多数开发机装过 Java 项目就有，省去手工摆 jar。
  const m2 = join(homedir(), '.m2', 'repository', 'commons-codec', 'commons-codec')
  if (existsSync(m2)) {
    const jars = readdirSync(m2)
      .filter((v) => /^\d/.test(v) && existsSync(join(m2, v, `commons-codec-${v}.jar`)))
      .sort((a, b) => a.localeCompare(b, undefined, { numeric: true }))
    if (jars.length) return join(m2, jars[jars.length - 1], `commons-codec-${jars[jars.length - 1]}.jar`)
  }
  return null
}

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

// crate 数量动态数：曾经 label 里硬编码「97 crates」，crate 增删后静默漂移
// （实测 96 个目录 + 根包）。label 只该描述现状，不该是一个要人维护的常数。
const RUST_CRATE_COUNT =
  1 + readdirSync(join(ROOT, 'oa4rust', 'backend', 'crates'), { withFileTypes: true })
    .filter((d) => d.isDirectory()).length

/** 某 Python 模块在当前 `python` 下是否可导入（用于 requires 守门，同 java 项范式）。 */
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
    // requires 守门（同 java 项范式）：解释器没装 pytest 时 SKIP 并说明，
    // 不像命令失败那样制造环境性红。本机 managed python 无 pytest；
    // 装了的开发机（或 CI 的 python）会自动接上这一项。
    requires: () => (hasPythonModule('pytest') ? true : '当前 python 无 pytest 模块'),
    timeout: 10 * 60 * 1000,
  },
  {
    id: 'java',
    label: 'cool (Java, -Xlint:all -Werror)',
    cwd: 'cool',
    cmd: (jar) =>
      `javac -Xlint:all -Werror -encoding UTF-8 -cp "${jar}" -d "${join(ROOT, 'cool', '.build')}" EncryptUtil.java`,
    pattern: PATTERNS.java,
    requires: () => {
      const jar = findCommonsCodec()
      return jar ? { jar } : 'commons-codec jar 未找到（放 cool/lib/ 或设 COMMONS_CODEC_JAR）'
    },
    timeout: 2 * 60 * 1000,
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
]

const args = process.argv.slice(2)
const verbose = args.includes('--verbose')
if (args.includes('--list')) {
  for (const c of CHECKS) console.log(`${c.id.padEnd(14)} ${c.label}`)
  process.exit(0)
}
const onlyArg = args.find((a) => a.startsWith('--only'))
const only = onlyArg
  ? (onlyArg.includes('=') ? onlyArg.split('=')[1] : args[args.indexOf(onlyArg) + 1]).split(',').map((s) => s.trim())
  : null

const selected = only ? CHECKS.filter((c) => only.includes(c.id)) : CHECKS
if (selected.length === 0) {
  console.error(`没有匹配的检查项。可用：${CHECKS.map((c) => c.id).join(', ')}`)
  process.exit(2)
}

const results = []
for (const check of selected) {
  // 不跑命令的静态检查（例如「成员是否都继承了 workspace lints」）
  if (check.custom) {
    process.stdout.write(`- ${check.label} … `)
    const outcome = check.custom()
    results.push({ ...check, status: outcome.ok ? 'PASS' : 'FAIL', hits: outcome.hits })
    console.log(outcome.ok ? 'PASS' : `FAIL (${outcome.hits.length} 处)`)
    if (!outcome.ok) for (const line of outcome.hits) console.log(`    ${line}`)
    continue
  }

  const skipped = check.requires ? check.requires() : null
  if (typeof skipped === 'string') {
    results.push({ ...check, status: 'SKIP', detail: skipped })
    console.log(`- ${check.label} … SKIP (${skipped})`)
    continue
  }

  const cmd = typeof check.cmd === 'function' ? check.cmd(skipped.jar) : check.cmd
  process.stdout.write(`- ${check.label} … `)
  const started = Date.now()
  const run = spawnSync(cmd, {
    cwd: join(ROOT, check.cwd),
    env,
    shell: true,
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
    timeout: check.timeout,
  })
  const output = `${run.stdout ?? ''}${run.stderr ?? ''}`.replace(ANSI, '')
  const hits = output.split(/\r?\n/).filter((line) => check.pattern.test(line))
  const seconds = ((Date.now() - started) / 1000).toFixed(1)
  const failed = run.status !== 0 || hits.length > 0
  results.push({ ...check, status: failed ? 'FAIL' : 'PASS', hits, exit: run.status, seconds })

  console.log(failed ? `FAIL (exit=${run.status}, 告警 ${hits.length} 条, ${seconds}s)` : `PASS (${seconds}s)`)
  if (verbose || failed) {
    for (const line of hits.slice(0, 40)) console.log(`    ${line.trim()}`)
    if (hits.length > 40) console.log(`    … 另有 ${hits.length - 40} 条`)
    if (failed && hits.length === 0) console.log(output.split(/\r?\n/).slice(-25).join('\n'))
  }
}

const failedCount = results.filter((r) => r.status === 'FAIL').length
const skippedCount = results.filter((r) => r.status === 'SKIP').length
console.log(`\n${results.length} 项检查：${results.length - failedCount - skippedCount} 通过 / ${failedCount} 失败 / ${skippedCount} 跳过`)
process.exit(failedCount === 0 ? 0 : 1)
