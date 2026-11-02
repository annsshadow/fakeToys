// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// ============================================================
// 签到中枢 前端逻辑：拉取 /api/stats、/api/trend、/api/records，
// 渲染指标 Bento、站点卡片、记录表，并用纯 SVG 手绘奖励趋势折线图。
// 第 57 轮：口令只从 URL 读一次——立即转入 sessionStorage 并用
// replaceState 抹掉地址栏查询串（token 不再留在浏览器历史/代理日志里）。
// 之后刷新/导航从 sessionStorage 取，URL 保持干净。
// ============================================================

const _URL_TOKEN = new URLSearchParams(location.search).get("token") || "";
let TOKEN = sessionStorage.getItem("panel_token") || _URL_TOKEN;
if (_URL_TOKEN) {
  sessionStorage.setItem("panel_token", _URL_TOKEN);
  try {
    history.replaceState(null, "", location.pathname + location.hash);
  } catch { /* 某些内嵌环境不允许 replaceState，忽略 */ }
}
const SITE_COLORS = ["#6d5efc", "#22d3ee", "#34e5a5", "#f0a640", "#ff5d73"];
const NS = "http://www.w3.org/2000/svg";
let trendDays = 30;

function api(path, opts = {}) {
  const headers = { ...(opts.headers || {}) };
  if (TOKEN) headers["X-Token"] = TOKEN;
  return fetch(path, { ...opts, headers });
}

const $ = (id) => document.getElementById(id);
const colorOf = (i) => SITE_COLORS[i % SITE_COLORS.length];

function fmtTime(iso, short = false) {
  if (!iso) return "—";
  const d = new Date(iso);
  return d.toLocaleString("zh-CN", short ? { month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit" } : { hour12: false });
}
function fmtNum(n, currency) {
  if (n === null || n === undefined) return "—";
  // 第 72 轮：currency 与数值同样来自远端站点响应（部分适配器从页面文本读余额），
  // 第 38 轮的 esc() 加固漏了这里——`${c}${n}` 是裸插值，站点可控值能进 innerHTML。
  const c = currency && currency !== "quota" ? esc(currency) : "";
  return `${c}${esc(n)}`;
}
function fmtDur(ms) {
  if (ms === null || ms === undefined || ms === 0) return "—";
  return ms < 1000 ? `${ms}ms` : `${(ms / 1000).toFixed(1)}s`;
}

const STATUS_TEXT = { success: "领取成功", already: "今日已领", failed: "失败", skipped: "已跳过" };

// ---------------- 状态灯 ----------------
function renderLive(data) {
  const dot = $("live-dot");
  const text = $("live-text");
  if (data.running) {
    dot.className = "live-dot running";
    text.textContent = "签到运行中…";
  } else {
    dot.className = "live-dot idle";
    text.textContent = data.nextRunAt
      ? `下次 ${fmtTime(data.nextRunAt, true)}`
      : (data.nextRunNote || "空闲待命");
  }
}

// ---------------- 顶部指标 Bento ----------------
function renderMetrics(data) {
  const t = data.totals;
  const cards = [
    { label: "今日已签", value: `${t.sitesCheckedInToday}`, unit: `/ ${t.sitesConfigured}`, foot: "已签 / 已配置站点", color: "#34e5a5", big: false },
    { label: "累计奖励", value: fmtNum(t.totalReward, "$"), foot: "历史领取总额", color: "#6d5efc", big: true },
    { label: "本周奖励", value: fmtNum(t.weekReward, "$"), foot: "本周（周一起）", color: "#22d3ee", big: false },
    { label: "本月奖励", value: fmtNum(t.monthReward, "$"), foot: "本月至今", color: "#22d3ee", big: false },
    { label: "签到次数", value: `${t.totalCheckins}`, foot: `共运行 ${t.totalRuns} 次`, color: "#f0a640", big: false },
    { label: "成功率", value: `${t.successRate}`, unit: "%", foot: `失败 ${t.failedRuns} 次`, color: "#34e5a5", big: false },
    { label: "平均耗时", value: fmtDur(t.avgDurationMs), foot: "每站单次均值", color: "#ff5d73", big: false },
  ];
  $("metrics").innerHTML = cards.map((c) => `
    <div class="metric ${c.big ? "big" : ""}" style="--accent:${c.color}">
      <div class="m-label"><span class="spark" style="--accent:${c.color}"></span>${c.label}</div>
      <div class="m-value">${c.value}${c.unit ? `<span class="unit">${c.unit}</span>` : ""}</div>
      <div class="m-foot">${c.foot}</div>
    </div>`).join("");
}

// 第 38 轮：面板此前把站点/记录字段直接 innerHTML 插值 —— 其中
// `s.lastMessage` / `r.message` 来自**远端站点响应**，站点返回
// `<img onerror=...>` 就能在本地面板执行脚本。所有不可信字符串进 HTML
// 前必须过 esc()（文本与双引号属性都安全）。
function esc(v) {
  return String(v ?? "").replace(/[&<>"']/g, (c) =>
    c === "&" ? "&amp;" : c === "<" ? "&lt;" : c === ">" ? "&gt;" :
    c === '"' ? "&quot;" : "&#39;"
  );
}

// ---------------- 站点卡片 ----------------
function renderCards(sites) {
  $("cards").innerHTML = sites.map((s, i) => {
    const status = s.checkedInToday ? (s.todayStatus || "success") : (s.todayStatus || "none");
    const badgeText = s.checkedInToday ? "✅ 今日已领" : (STATUS_TEXT[s.todayStatus] || "未签到");
    const color = colorOf(i);
    const shot = s.lastScreenshot ? `<a class="shot-link" href="/${esc(s.lastScreenshot)}" target="_blank">查看截图</a>` : "";
    return `
    <div class="card">
      <div class="glow" style="background:${color}"></div>
      <div class="c-head">
        <span class="c-name"><span class="c-ico" style="background:linear-gradient(135deg,${color},${colorOf(i + 1)})">${esc(s.siteName)[0]}</span>${esc(s.siteName)}</span>
        <span class="badge ${status}">${badgeText}</span>
      </div>
      <div class="c-grid">
        <div class="cell"><div class="k">今日奖励</div><div class="v">${fmtNum(s.todayReward, s.currency)}</div></div>
        <div class="cell"><div class="k">当前余额</div><div class="v">${fmtNum(s.latestBalance, s.currency)}</div></div>
        <div class="cell"><div class="k">累计领取</div><div class="v">${fmtNum(s.totalReward, s.currency)}</div></div>
        <div class="cell"><div class="k">本周 / 本月</div><div class="v">${fmtNum(s.weekReward, s.currency)} <small>/ ${fmtNum(s.monthReward, s.currency)}</small></div></div>
        <div class="cell"><div class="k">连续签到</div><div class="v">${s.streak}<small> 天 · 最长 ${s.bestStreak}</small></div></div>
        <div class="cell"><div class="k">最近耗时</div><div class="v">${fmtDur(s.lastDurationMs)}<small> · 均 ${fmtDur(s.avgDurationMs)}</small></div></div>
        <div class="cell" style="grid-column:span 2">
          <div class="k">成功率 · ${s.successRate}%（${s.totalRuns - s.skippedRuns - s.failedRuns}/${s.totalRuns - s.skippedRuns} 次）</div>
          <div class="rate-bar"><i style="width:${s.successRate}%"></i></div>
        </div>
      </div>
      <div class="c-foot">
        <span class="c-msg" title="${esc(s.lastMessage)}">${s.lastMessage ? esc(s.lastMessage) : "尚无运行记录"}${shot}</span>
        <button class="c-btn" data-site="${s.siteId}">单独签到</button>
      </div>
    </div>`;
  }).join("");

  $("cards").querySelectorAll("[data-site]").forEach((btn) => {
    btn.addEventListener("click", async () => {
      btn.disabled = true; btn.textContent = "运行中…";
      await api(`/api/run/${btn.dataset.site}`, { method: "POST" });
      setTimeout(refresh, 1500);
    });
  });
}

// ---------------- 记录表 ----------------
// 第 71 轮：状态筛选走服务端 ?status=（面板只展示筛选后的结果，
// 不是在前端对最近 80 条再截一刀——失败记录可能被成功流水挤出窗口）。
let recordStatus = "";

async function loadRecords() {
  const qs = new URLSearchParams({ limit: "80" });
  if (recordStatus) qs.set("status", recordStatus);
  const res = await api(`/api/records?${qs}`);
  if (!res.ok) return;
  const records = await res.json();
  const suffix = recordStatus ? ` · ${STATUS_TEXT[recordStatus] || recordStatus}` : "";
  $("rec-count").textContent = `最近 ${records.length} 条${suffix}`;
  const body = $("records-body");
  if (!records.length) { body.innerHTML = `<tr><td colspan="7" class="empty">暂无记录，点「全部签到」开始</td></tr>`; return; }
  body.innerHTML = records.map((r) => `
    <tr>
      <td>${fmtTime(r.timestamp)}</td>
      <td>${esc(r.siteName)}</td>
      <td><span class="st ${r.status}">${STATUS_TEXT[r.status] || r.status}</span></td>
      <td>${fmtNum(r.reward, r.currency)}</td>
      <td>${fmtNum(r.balance, r.currency)}</td>
      <td>${fmtDur(r.durationMs)}</td>
      <td>${esc(r.message)}</td>
    </tr>`).join("");
}

// ---------------- SVG 趋势折线图 ----------------
async function loadTrend() {
  const res = await api(`/api/trend?days=${trendDays}`);
  if (!res.ok) return;
  const data = await res.json();
  drawChart(data);
  renderLegend(data.sites);
}

function renderLegend(sites) {
  $("legend").innerHTML = sites.map((s, i) =>
    `<div class="item"><span class="swatch" style="background:${colorOf(i)}"></span>${esc(s.name)}</div>`
  ).join("") + `<div class="item"><span class="swatch" style="background:#fff"></span>总额</div>`;
}

function drawChart(data) {
  const svg = $("chart");
  svg.innerHTML = "";
  const rect = svg.getBoundingClientRect();
  const W = rect.width || 900, H = rect.height || 240;
  const pad = { l: 44, r: 16, t: 16, b: 26 };
  const iw = W - pad.l - pad.r, ih = H - pad.t - pad.b;
  const pts = data.points;
  if (!pts.length) return;

  const maxVal = Math.max(0.0001, ...pts.map((p) => p.total));
  const niceMax = niceCeil(maxVal);
  const x = (i) => pad.l + (pts.length === 1 ? iw / 2 : (i / (pts.length - 1)) * iw);
  const y = (v) => pad.t + ih - (v / niceMax) * ih;

  // 网格 + Y 轴刻度
  const ticks = 4;
  for (let t = 0; t <= ticks; t++) {
    const val = (niceMax / ticks) * t;
    const yy = y(val);
    svg.appendChild(el("line", { x1: pad.l, y1: yy, x2: W - pad.r, y2: yy, stroke: "rgba(255,255,255,0.06)", "stroke-width": 1 }));
    svg.appendChild(el("text", { x: pad.l - 8, y: yy + 4, "text-anchor": "end", fill: "#4a5470", "font-size": 10 }, `$${round(val)}`));
  }
  // X 轴日期（稀疏标注）
  const step = Math.max(1, Math.floor(pts.length / 6));
  pts.forEach((p, i) => {
    if (i % step === 0 || i === pts.length - 1) {
      svg.appendChild(el("text", { x: x(i), y: H - 8, "text-anchor": "middle", fill: "#4a5470", "font-size": 10 }, p.date.slice(5)));
    }
  });

  // 各站点折线
  data.sites.forEach((site, si) => {
    const color = colorOf(si);
    const path = pts.map((p, i) => `${i === 0 ? "M" : "L"}${x(i)},${y(p.perSite[site.id] || 0)}`).join(" ");
    svg.appendChild(el("path", { d: path, fill: "none", stroke: color, "stroke-width": 2, "stroke-linejoin": "round", opacity: 0.85 }));
  });

  // 总额线（白色高亮 + 渐变面积）
  const gid = "areaGrad";
  const defs = el("defs", {});
  const grad = el("linearGradient", { id: gid, x1: 0, y1: 0, x2: 0, y2: 1 });
  grad.appendChild(el("stop", { offset: "0%", "stop-color": "rgba(255,255,255,0.22)" }));
  grad.appendChild(el("stop", { offset: "100%", "stop-color": "rgba(255,255,255,0)" }));
  defs.appendChild(grad); svg.appendChild(defs);
  const line = pts.map((p, i) => `${i === 0 ? "M" : "L"}${x(i)},${y(p.total)}`).join(" ");
  svg.appendChild(el("path", { d: `${line} L${x(pts.length - 1)},${y(0)} L${x(0)},${y(0)} Z`, fill: `url(#${gid})`, stroke: "none" }));
  svg.appendChild(el("path", { d: line, fill: "none", stroke: "#fff", "stroke-width": 2.5, "stroke-linejoin": "round" }));

  // 悬浮交互
  const tip = $("chart-tip");
  const hover = el("line", { x1: 0, y1: pad.t, x2: 0, y2: pad.t + ih, stroke: "rgba(255,255,255,0.3)", "stroke-width": 1, opacity: 0 });
  svg.appendChild(hover);
  const dots = data.sites.map((_, si) => {
    const c = el("circle", { r: 4, fill: colorOf(si), stroke: "#06070d", "stroke-width": 2, opacity: 0 });
    svg.appendChild(c); return c;
  });
  const totalDot = el("circle", { r: 4.5, fill: "#fff", stroke: "#06070d", "stroke-width": 2, opacity: 0 });
  svg.appendChild(totalDot);

  svg.onmousemove = (e) => {
    const r = svg.getBoundingClientRect();
    const mx = e.clientX - r.left;
    let idx = Math.round(((mx - pad.l) / iw) * (pts.length - 1));
    idx = Math.max(0, Math.min(pts.length - 1, idx));
    const p = pts[idx];
    hover.setAttribute("x1", x(idx)); hover.setAttribute("x2", x(idx)); hover.setAttribute("opacity", 1);
    data.sites.forEach((site, si) => {
      dots[si].setAttribute("cx", x(idx)); dots[si].setAttribute("cy", y(p.perSite[site.id] || 0)); dots[si].setAttribute("opacity", 1);
    });
    totalDot.setAttribute("cx", x(idx)); totalDot.setAttribute("cy", y(p.total)); totalDot.setAttribute("opacity", 1);
    tip.style.opacity = 1;
    tip.style.left = `${x(idx)}px`;
    tip.style.top = `${y(p.total)}px`;
    tip.innerHTML = `<div class="tip-date">${p.date}</div>` +
      data.sites.map((site, si) => `<div class="tip-row"><span class="tip-dot" style="background:${colorOf(si)}"></span>${esc(site.name)}: $${p.perSite[site.id] || 0}</div>`).join("") +
      `<div class="tip-row"><span class="tip-dot" style="background:#fff"></span><b>总额: $${p.total}</b></div>`;
  };
  svg.onmouseleave = () => {
    tip.style.opacity = 0; hover.setAttribute("opacity", 0);
    dots.forEach((d) => d.setAttribute("opacity", 0)); totalDot.setAttribute("opacity", 0);
  };
}

function el(tag, attrs, text) {
  const e = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
  if (text !== undefined) e.textContent = text;
  return e;
}
function niceCeil(v) {
  const pow = Math.pow(10, Math.floor(Math.log10(v)));
  const n = v / pow;
  const step = n <= 1 ? 1 : n <= 2 ? 2 : n <= 5 ? 5 : 10;
  return step * pow;
}
function round(v) { return v >= 10 ? Math.round(v) : Number(v.toFixed(2)); }

// ---------------- 主流程 ----------------
async function loadStats() {
  const res = await api("/api/stats");
  if (res.status === 401) {
    // 第 57 轮：不再引导用户把口令拼进 URL（会进代理日志/浏览器历史）。
    const input = window.prompt("请输入访问口令：");
    if (input && input.trim()) {
      sessionStorage.setItem("panel_token", input.trim());
      location.reload();
    } else {
      $("live-text").textContent = "需口令：请在弹窗中输入";
    }
    return;
  }
  const data = await res.json();
  renderLive(data);
  renderMetrics(data);
  renderCards(data.sites);
}

function refresh() { loadStats(); loadRecords(); loadTrend(); }

$("refresh").addEventListener("click", refresh);
$("run-all").addEventListener("click", async (e) => {
  const b = e.currentTarget; b.disabled = true; b.innerHTML = "<span>⏳</span> 运行中…";
  await api("/api/run", { method: "POST" });
  setTimeout(() => { b.disabled = false; b.innerHTML = "<span>⚡</span> 全部签到"; refresh(); }, 2000);
});
// 第 71 轮：记录状态筛选片（服务端过滤，切换后立即重载记录表）
$("rec-filters").querySelectorAll("[data-status]").forEach((btn) => {
  btn.addEventListener("click", () => {
    recordStatus = btn.dataset.status || "";
    $("rec-filters").querySelectorAll("button").forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    loadRecords();
  });
});
$("range").querySelectorAll("[data-days]").forEach((btn) => {
  btn.addEventListener("click", () => {
    $("range").querySelectorAll("button").forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    trendDays = Number(btn.dataset.days);
    loadTrend();
  });
});
window.addEventListener("resize", () => loadTrend());

refresh();
setInterval(refresh, 15000);
