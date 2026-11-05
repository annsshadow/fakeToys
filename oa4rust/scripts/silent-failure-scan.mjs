#!/usr/bin/env node
// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// 「把加载失败折叠成空态」横切扫描（R113）。
//
// 只认脚本区真实代码：注释与字符串内容先被抹成空格，避免把
// `// 旧实现 catch { rows.value = [] }` 这类说明文字算成缺陷。
// 分类：
//   listFold  —— load/fetch/refresh 类函数里的 catch 把列表 state 置空/null，
//                或 catch 直接 return [] / 返回 {ok:true, rows:[]}（待修/已修）
//   otherFold —— catch 把非列表表单 state 置 null（交互失败重置，不进待修清单）
//   emptyOnly —— 模板用 length===0 判空、整文件却没有任何失败/重试提示（疑似）
//
// 断言下限用于防止扫描范围被改窄后「看起来干净」。数字取实测值的保守下界。

import { readdirSync, readFileSync, statSync } from 'node:fs'
import { dirname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..')
const ROOTS = [
  { name: 'mobile', dir: join(repoRoot, 'oa4rust/mobile/src') },
  { name: 'desktop', dir: join(repoRoot, 'oa4rust/frontend/apps/desktop/src') },
]

// 已修复清单：R105/R108/R109/R110/R111/R112 的列表加载路径，必须零 listFold。
const FIXED = {
  'oa4rust/mobile/src/pages/bbs/bbs.vue': 'R105',
  'oa4rust/mobile/src/pages/bbs/bbsFeed.ts': 'R105',
  'oa4rust/mobile/src/pages/chat/chat.vue': 'R108',
  'oa4rust/mobile/src/pages/chat/chatHistory.ts': 'R108',
  'oa4rust/mobile/src/pages/process/processList.ts': 'R109',
  'oa4rust/mobile/src/pages/message/message.vue': 'R110',
  'oa4rust/mobile/src/pages/message/messageFeed.ts': 'R110',
  'oa4rust/mobile/src/pages/collect/collect.vue': 'R111',
  'oa4rust/mobile/src/pages/collect/collectFeed.ts': 'R111',
  'oa4rust/mobile/src/pages/browse/browse.vue': 'R112',
  'oa4rust/mobile/src/pages/browse/browseFeed.ts': 'R112',
  'oa4rust/mobile/src/utils/loadList.ts': 'R113',
  'oa4rust/mobile/src/pages/contacts/contacts.vue': 'R113',
  'oa4rust/mobile/src/pages/meeting/meeting.vue': 'R113',
  'oa4rust/mobile/src/pages/recycle/recycle.vue': 'R113',
  'oa4rust/mobile/src/pages/index/index.vue': 'R113',
  'oa4rust/mobile/src/pages/attendance/attendance.vue': 'R114',
  'oa4rust/mobile/src/pages/calendar/calendar.vue': 'R114',
  'oa4rust/mobile/src/pages/doc/doc.vue': 'R114',
  'oa4rust/mobile/src/pages/process/process.vue': 'R114',
  'oa4rust/frontend/apps/desktop/src/views/EmptyApp.vue': 'R114',
  // R115：桌面 32 个同构生成页 + Dashboard 接入统一失败口径（useListError / ListLoadError）。
  'oa4rust/frontend/apps/desktop/src/views/AnnApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsColumnApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsColumnManagerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsDictDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsFormDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsIndexApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsModuleApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsViewDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CmsXformApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CollectApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/CommonApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/Dashboard.vue': 'R115/R117',
  'oa4rust/frontend/apps/desktop/src/views/ConfigDesignerApp.vue': 'R116',
  'oa4rust/frontend/apps/desktop/src/views/DeploymentApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/FaceSetApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/FtSearchApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/LogViewerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/NoteApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/PortalDictDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/PortalPageDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/PortalWidgetDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessApplicationApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessDictDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessFormDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessManagerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessTaskCenterApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ProcessXformApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/QueryExplorerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/QueryImporterDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/QueryQueryApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/QueryStatDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ServiceInvokeDesignerApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/TemplateApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/ThreeMemberApp.vue': 'R115',
  'oa4rust/frontend/apps/desktop/src/views/AppInfoApp.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/CategoryApp.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/FileInfoApp.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/RoleManager.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/UnitApp.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/ViewApp.vue': 'R118',
  'oa4rust/frontend/apps/desktop/src/views/BamApp.vue': 'R119',
  'oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue': 'R119',
  'oa4rust/frontend/apps/desktop/src/views/FindDesignerApp.vue': 'R119',
  'oa4rust/frontend/apps/desktop/src/views/HomepageApp.vue': 'R119',
  'oa4rust/frontend/apps/desktop/src/views/QueryManager.vue': 'R119',
  'oa4rust/frontend/apps/desktop/src/views/DocumentApp.vue': 'R120',
  'oa4rust/frontend/apps/desktop/src/views/HotpicApp.vue': 'R120',
  'oa4rust/frontend/apps/desktop/src/views/PortalApp.vue': 'R120',
  'oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue': 'R121',
  'oa4rust/frontend/apps/desktop/src/views/QueryManagerDeep.vue': 'R121',
  'oa4rust/frontend/apps/desktop/src/views/Personal.vue': 'R122',
  'oa4rust/frontend/apps/desktop/src/views/QueryDesigner.vue': 'R122',
  'oa4rust/frontend/apps/desktop/src/views/QueryStatementDesigner.vue': 'R122',
  'oa4rust/frontend/apps/desktop/src/views/QueryViewApp.vue': 'R122',
  'oa4rust/frontend/apps/desktop/src/views/AIChatApp.vue': 'R123',
  'oa4rust/frontend/apps/desktop/src/views/FileManager.vue': 'R123',
  'oa4rust/frontend/apps/desktop/src/views/FormApp.vue': 'R124',
  'oa4rust/frontend/apps/desktop/src/views/FormDesigner.vue': 'R124',
  'oa4rust/frontend/apps/desktop/src/views/JPushApp.vue': 'R124',
  'oa4rust/frontend/apps/desktop/src/views/RecycleApp.vue': 'R126',
  'oa4rust/mobile/src/pages/process/start.vue': 'R125',
  'oa4rust/mobile/src/pages/search/search.vue': 'R125',
}

const LIST_STATE =
  /^(rows|list|items|data|records|result|results|msgs|messages|conversations|events|calendars|topics|detail|notices|attachRows|myDetails|calendars)/i
const MIN_SCANNED_FILES = 120
// 随修复推进，fold 候选数量会逐批下降；保留一个下限只用于发现「候选提取失效」。
// R126 修复 maskSource（.vue 只看 <script>）后实测候选 53（handled 3 + other 50），取下限 40。
const MIN_FOLD_CANDIDATES = 40

// 首轮扫描（R113 建立）确认的待修文件；修好一个就移到 FIXED 并从这里删掉，
// 新出现的文件必须显式登记，否则守卫失败。
const BASELINE_PENDING = new Set([])

// 已定性但确认不需要改的空态页：文件 → [类别, 依据]。
//   b = 列表取数失败已有可见兜底（toast / 上层错误态，写明位置）
//   c = 页面没有列表取数，空态来自常量或静态配置
// 三类（a=FIXED、b/c=CLASSIFIED）都显式登记，复查时每一行都能说出「为什么不用改」。
const CLASSIFIED = new Map([
  // 见 R115 台账：b/c 类逐页证据在此登记。
])

// 已定性为 a 类（有取数、列表失败会被展示成空态）但尚未修复：R115 扫描口径收紧后新发现。
// 与 FIXED 分开计数，下一批逐个替换后从本表移到 FIXED；不允许无理由留在本表。
const A_PENDING = new Map([])

function walk(dir, out = []) {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry)
    const st = statSync(full)
    if (st.isDirectory()) {
      if (entry === 'node_modules' || entry === 'dist') continue
      walk(full, out)
    } else if (/\.(vue|ts)$/.test(entry) && !/\.test\.ts$/.test(entry) && !entry.endsWith('.d.ts')) {
      out.push(full)
    }
  }
  return out
}

/** 注释与字符串内容抹成空格（保留长度与换行），用于脚本区模式匹配。 */
function maskSource(rawSrc, isVue) {
  // .vue 只保留 <script> 块原文，模板/HTML 先抹成空格：否则模板文案里的 `/*`
  // 会被当成未闭合块注释，吞掉整个 <script setup>（AIChatApp 实测漏抓）。
  let src = rawSrc
  if (isVue) {
    const kept = rawSrc.split('').map((c) => (c === '\n' ? '\n' : ' '))
    const re = /<script\b[^>]*>([\s\S]*?)<\/script>/gi
    for (const m of rawSrc.matchAll(re)) {
      const bodyStart = m.index + m[0].indexOf('>') + 1
      for (let i = 0; i < m[1].length; i++) kept[bodyStart + i] = rawSrc[bodyStart + i]
    }
    src = kept.join('')
  }
  const out = src.split('')
  let state = 'code'
  let quote = ''
  for (let i = 0; i < src.length; i++) {
    const c = src[i]
    const next = src[i + 1]
    if (state === 'code') {
      if (c === '/' && next === '/') {
        state = 'line'
        out[i] = ' '
        out[i + 1] = ' '
        i++
      } else if (c === '/' && next === '*') {
        state = 'block'
        out[i] = ' '
        out[i + 1] = ' '
        i++
      } else if (c === "'" || c === '"' || c === '`') {
        state = 'string'
        quote = c
      }
    } else if (state === 'line') {
      if (c === '\n') state = 'code'
      else out[i] = ' '
    } else if (state === 'block') {
      out[i] = c === '\n' ? '\n' : ' '
      if (c === '*' && next === '/') {
        out[i + 1] = ' '
        i++
        state = 'code'
      }
    } else if (state === 'string') {
      if (c === '\\') {
        out[i] = ' '
        if (i + 1 < src.length) out[i + 1] = ' '
        i++
      } else if (c === quote) {
        state = 'code'
      } else if (c !== '\n') {
        out[i] = ' '
      }
    }
  }
  return out.join('')
}

function lineOf(src, index) {
  let line = 1
  for (let i = 0; i < index; i++) if (src.charCodeAt(i) === 10) line++
  return line
}

function findCatchBlocks(masked) {
  const blocks = []
  const re = /\bcatch\s*(?:\([^)]*\))?\s*\{/g
  for (const m of masked.matchAll(re)) {
    const open = m.index + m[0].length - 1
    let depth = 0
    let close = -1
    for (let i = open; i < masked.length; i++) {
      if (masked[i] === '{') depth++
      else if (masked[i] === '}') {
        depth--
        if (depth === 0) {
          close = i
          break
        }
      }
    }
    if (close === -1) continue
    blocks.push({ start: m.index, body: masked.slice(open + 1, close) })
  }
  return blocks
}

function enclosingName(masked, index) {
  const before = masked.slice(0, index)
  const fns = [...before.matchAll(/\b(?:async\s+)?function\s+([A-Za-z0-9_$]+)/g)]
  const consts = [...before.matchAll(/\bconst\s+([A-Za-z0-9_$]+)\s*=\s*(?:async\s*)?(?:\(|<)/g)]
  const methods = [...before.matchAll(/\b(?:async\s+)?([A-Za-z0-9_$]+)\s*\([^)]*\)\s*\{/g)]
  const candidates = [
    fns.length ? { name: fns[fns.length - 1][1], index: fns[fns.length - 1].index } : null,
    consts.length ? { name: consts[consts.length - 1][1], index: consts[consts.length - 1].index } : null,
    methods.length ? { name: methods[methods.length - 1][1], index: methods[methods.length - 1].index } : null,
  ].filter(Boolean)
  candidates.sort((a, b) => b.index - a.index)
  return candidates.length ? candidates[0].name : ''
}

function inspect(file) {
  const src = readFileSync(file, 'utf8')
  const masked = maskSource(src, file.endsWith('.vue'))
  const rel = relative(repoRoot, file).replaceAll('\\', '/')
  const hits = []
  for (const block of findCatchBlocks(masked)) {
    const assigns = [...block.body.matchAll(/([A-Za-z0-9_$]+)\s*\.\s*value\s*=\s*(?:\[\s*\]|null)/g)]
    const returnsEmpty = /\breturn\s+\[\s*\]/.test(block.body)
    const returnsOkEmpty = /\breturn\s*\{\s*ok\s*:\s*true\s*,\s*rows\s*:\s*\[\s*\]\s*\}/.test(block.body)
    // 本地解析兜底：紧邻的 try 只做 JSON.parse，catch 返回 [] 是「坏数据默认值」，
    // 不是取数失败，不能算静默折叠（如 FormDesigner 的字段选项解析）。
    const beforeBlock = masked.slice(0, block.start)
    const lastTry = [...beforeBlock.matchAll(/(?<![A-Za-z0-9_$])try\s*\{/g)].pop()
    const localParse = Boolean(lastTry) && /JSON\.parse\s*\(/.test(beforeBlock.slice(lastTry.index))
    if (!assigns.length && !returnsOkEmpty && localParse) continue
    if (!assigns.length && !returnsEmpty && !returnsOkEmpty) continue
    const fn = enclosingName(masked, block.start)
    const listAssign = assigns.find((a) => LIST_STATE.test(a[1]))
    // catch 里同时写了 error ref（如 loadError/historyError）或 toast/throw，说明失败对用户可见，
    // 清空列表属于「首屏失败」而非静默折叠。
    const hasErrorSignal =
      /[A-Za-z0-9_$]*error[A-Za-z0-9_$]*\s*\.\s*value\s*=/i.test(block.body) ||
      /(uni\.)?showToast|uni\.showModal|\bthrow\b/.test(block.body)
    const fold = returnsOkEmpty || returnsEmpty || Boolean(listAssign)
    const kind = !fold ? 'otherFold' : hasErrorSignal ? 'handledFold' : 'silentFold'
    hits.push({
      line: lineOf(src, block.start),
      fn,
      kind,
      detail: returnsOkEmpty
        ? 'return {ok:true,rows:[]}'
        : returnsEmpty
          ? 'return []'
          : `${(listAssign ?? assigns[0])[1]}.value = 空/null`,
    })
  }

  // 模板空态：有 length===0 判空与「暂无/无数据」文案，但整文件无失败/重试提示。
  // 判据必须盯住「列表失败是否可见」：只认统一口径标识（loadError/ListLoadError/pendingError）
  // 或错误态里的「重试」。旧版把「错误/失败/showToast」算作兜底，只要页面在别处有
  // toast.error（如删除/详情失败）就会被误判为已处理——这正是 R115 前清单漏抓的根因。
  const emptyMatch = /length\s*===\s*0/.test(src) && /(暂无|无数据|没有)/.test(src)
  const hasErrorUi = /(loadError|ListLoadError|pendingError|重试)/.test(src)
  const emptyOnly = Boolean(emptyMatch && !hasErrorUi)

  return { rel, hits, emptyOnly }
}

const files = ROOTS.flatMap((r) => walk(r.dir))
const results = files.map(inspect)
const withFile = (kind) =>
  results.flatMap((r) => r.hits.filter((h) => h.kind === kind).map((h) => ({ ...h, file: r.rel })))
const silentFold = withFile('silentFold')
const handledFold = withFile('handledFold')
const otherFold = withFile('otherFold')
const emptyOnly = results.filter((r) => r.emptyOnly).map((r) => r.rel)
const classifiedEmptyOnly = emptyOnly.filter((file) => CLASSIFIED.has(file))
const pendingClassA = emptyOnly.filter((file) => A_PENDING.has(file))
const unclassifiedEmptyOnly = emptyOnly.filter((file) => !FIXED[file] && !CLASSIFIED.has(file) && !A_PENDING.has(file))
const fixedStillEmptyOnly = emptyOnly.filter((file) => FIXED[file])

const fixedHits = silentFold.filter((h) => FIXED[h.file])
const pendingHits = silentFold.filter((h) => !FIXED[h.file])
const pendingFiles = [...new Set(pendingHits.map((h) => h.file))]

const summary = {
  scannedFiles: files.length,
  silentFoldHits: silentFold.length,
  fixedHits: fixedHits.length,
  pendingHits: pendingHits.length,
  handledFoldHits: handledFold.length,
  otherFoldHits: otherFold.length,
  emptyOnlyFiles: emptyOnly.length,
  classifiedEmptyOnlyFiles: classifiedEmptyOnly.length,
  pendingClassAFiles: pendingClassA.length,
  unclassifiedEmptyOnlyFiles: unclassifiedEmptyOnly.length,
}

const problems = []
if (files.length < MIN_SCANNED_FILES)
  problems.push(`扫描文件数 ${files.length} < ${MIN_SCANNED_FILES}，扫描范围疑似被改窄`)
if (silentFold.length + handledFold.length + otherFold.length < MIN_FOLD_CANDIDATES)
  problems.push(
    `catch 折叠候选 ${silentFold.length + handledFold.length + otherFold.length} < ${MIN_FOLD_CANDIDATES}，疑似漏扫`,
  )
if (fixedHits.length > 0)
  problems.push(`已修复文件仍有静默折叠：${fixedHits.map((h) => `${h.file}:${h.line}`).join(', ')}`)
for (const file of Object.keys(FIXED)) {
  if (!files.some((f) => relative(repoRoot, f).replaceAll('\\', '/') === file))
    problems.push(`已修复清单文件不存在：${file}`)
}
for (const file of pendingFiles) {
  if (!BASELINE_PENDING.has(file)) problems.push(`新发现未登记的静默折叠：${file}（请先修或登记到 BASELINE_PENDING）`)
}
for (const file of BASELINE_PENDING) {
  if (!files.some((f) => relative(repoRoot, f).replaceAll('\\', '/') === file))
    problems.push(`基线待修复文件不存在：${file}`)
}
for (const file of fixedStillEmptyOnly) problems.push(`已修复文件仍只有空态、无失败提示：${file}（R115 口径未生效）`)
for (const file of unclassifiedEmptyOnly)
  problems.push(`未定性的空态页：${file}（请修成 FIXED，或登记到 CLASSIFIED 并写明 b/c 依据）`)
for (const file of CLASSIFIED.keys()) {
  if (!files.some((f) => relative(repoRoot, f).replaceAll('\\', '/') === file))
    problems.push(`已定性清单文件不存在：${file}`)
}
for (const file of A_PENDING.keys()) {
  if (!files.some((f) => relative(repoRoot, f).replaceAll('\\', '/') === file))
    problems.push(`待修 a 类清单文件不存在：${file}`)
}

if (process.argv.includes('--json')) {
  console.log(
    JSON.stringify(
      {
        summary,
        fixedHits,
        pendingHits: pendingHits.map(({ file, line, fn, detail }) => ({ file, line, fn, detail })),
        otherFold,
        emptyOnly,
      },
      null,
      2,
    ),
  )
} else {
  console.log('== 汇总 ==')
  console.log(JSON.stringify(summary, null, 2))
  console.log('\n== 待修复（listFold）==')
  for (const hit of pendingHits) console.log(`${hit.file}:${hit.line}  [${hit.fn || '?'}]  ${hit.detail}`)
  console.log('\n== 已修复文件里的残留（应为 0）==')
  for (const hit of fixedHits) console.log(`${hit.file}:${hit.line}  [${hit.fn || '?'}]  ${hit.detail}`)
  console.log('\n== 疑似只有空态、无任何失败提示（未定性，应为 0）==')
  for (const file of unclassifiedEmptyOnly) console.log(file)
  console.log('\n== 已定性为空态无需改（b=已有兜底 / c=纯静态）==')
  for (const file of classifiedEmptyOnly) {
    const [kind, reason] = CLASSIFIED.get(file)
    console.log(`${kind}  ${file}  — ${reason}`)
  }
  console.log('\n== 已定性为 a 类（有取数、失败会变空态）· 待修复 ==')
  for (const file of pendingClassA) console.log(`a   ${file}  — ${A_PENDING.get(file)}`)
  if (problems.length) {
    console.log('\n== 守卫失败 ==')
    for (const p of problems) console.log(`- ${p}`)
    process.exit(2)
  }
}
