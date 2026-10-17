// 依赖漏洞闸门。CI 的 dependency-audit job 与本文件的测试
// （audit-gate.test.cjs）共用这一份实现——逻辑只有一处，
// 不会出现「改了闸门、测试还在验旧逻辑」的情况。
//
// 用法：
//   node audit-gate.cjs check-counts <project> <audit.json> <baseline>
//   node audit-gate.cjs check-overrides <pnpm-workspace.yaml> <baseline>
//
// 退出码 0 = 放行；1 = 应当拦住（CI 里直接 ::error:: 报错）。

const fs = require("fs");

// Windows PowerShell 5.1 的 `>` 重定向写的是 UTF-16LE，CI 的 bash 写 UTF-8。
// 两种都得能读，否则本地根本验不了 CI 那一步——而「本地验不了」等于
// 这个闸门没被验证过。
function readText(p) {
  const buf = fs.readFileSync(p);
  if (buf.length >= 2 && buf[0] === 0xff && buf[1] === 0xfe) {
    return buf.toString("utf16le").replace(/^﻿/, "");
  }
  if (buf.length >= 3 && buf[0] === 0xef && buf[1] === 0xbb && buf[2] === 0xbf) {
    return buf.toString("utf8").replace(/^﻿/, "");
  }
  return buf.toString("utf8");
}

function parseBaseline(text) {
  // 形如
  //   [miniapp]
  //   critical = 3
  //   total = 39
  const out = {};
  let cur = null;
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    const sec = line.match(/^\[([^\]]+)\]$/);
    if (sec) {
      cur = sec[1];
      out[cur] = out[cur] || {};
      continue;
    }
    const kv = line.match(/^(\S+)\s*=\s*(\S+)$/);
    if (kv && cur) out[cur][kv[1]] = kv[2];
  }
  return out;
}

// 审计计数闸门：**上限**，不是「必须 0」。
// 剩下的漏洞全挂在 @dcloudio 构建链/测试链上，实测产物里一个都没有；
// 强推版本会打破 @dcloudio 的精确 pin、构建假红。
// 涨了红（来了新漏洞，或有人删了 overrides），降了提示去更新基线。
function checkCounts(project, auditJson, baseTxt) {
  const cur = JSON.parse(auditJson).metadata.vulnerabilities;
  const base = parseBaseline(baseTxt)[project];
  if (!base || base.total === undefined) {
    return { fail: true, msg: `基线文件里没有 [${project}] 段或缺 total 行` };
  }
  const limit = parseInt(base.total, 10);
  const total = cur.critical + cur.high + cur.moderate + cur.low;
  const detail =
    `critical=${cur.critical} high=${cur.high} moderate=${cur.moderate} low=${cur.low}`;

  if (total > limit) {
    return {
      fail: true,
      msg: `${project} 漏洞数 ${total} 超过基线 ${limit}（${detail}）`,
      hint: [
        "新增的多半来自 @dcloudio 构建链。先确认它没进 dist/build 产物，",
        "再决定是加 pnpm-workspace.yaml 的 overrides 还是等上游发版。",
      ],
    };
  }
  if (total < limit) {
    return {
      fail: false,
      notice: `${project} 漏洞数降到 ${total}（基线 ${limit}）——请同步更新 audit-baseline.ini，`
        + "否则这次修复没有留下守卫",
    };
  }
  return { fail: false, msg: `${project}: ${total} 条，与基线持平（${detail}）` };
}

// overrides 在位闸门。防的是「有人为了让某个包装得上而删了 override」——
// 那种情况计数可能刚好还在基线内，但已经名不副实。
function checkOverrides(yamlTxt, baseTxt) {
  const wanted = Object.entries(parseBaseline(baseTxt)["miniapp.overrides"] || {});
  if (wanted.length === 0) {
    return { fail: true, msg: "基线文件里没有 [miniapp.overrides] 段——闸门无从校验" };
  }
  const missing = wanted.filter(([pkg, ver]) => {
    const re = new RegExp('"' + pkg.replace(/[/@]/g, "\\$&") + '":\\s*"?' + ver + '"?');
    return !re.test(yamlTxt);
  });
  if (missing.length > 0) {
    return {
      fail: true,
      msg: `override 不在 pnpm-workspace.yaml 里了: ${missing.map((m) => m[0]).join(", ")}`,
      hint: [
        "删掉 override 会让 miniapp 漏洞数涨回 58（3/18/28/9）。",
        "确认是故意为之再删，并同步改 audit-baseline.ini。",
      ],
    };
  }
  return { fail: false, msg: `overrides 齐全（${wanted.length} 条）` };
}

function emit(res, label) {
  if (res.msg) console.log(`${label}: ${res.msg}`);
  if (res.notice) console.log(`::notice::${res.notice}`);
  if (res.hint) for (const h of res.hint) console.log(`::error::${h}`);
  if (res.fail) console.log(`::error::${res.msg}`);
  return res.fail ? 1 : 0;
}

if (require.main === module) {
  const [cmd, a, b, c] = process.argv.slice(2);
  let res;
  if (cmd === "check-counts") {
    res = checkCounts(a, readText(b), readText(c));
  } else if (cmd === "check-overrides") {
    res = checkOverrides(readText(a), readText(b));
  } else {
    console.error("用法: node audit-gate.cjs check-counts|check-overrides ...");
    process.exit(2);
  }
  process.exit(emit(res, "audit-gate"));
}

module.exports = { parseBaseline, checkCounts, checkOverrides };
