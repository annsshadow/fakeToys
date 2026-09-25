# 公益站自动签到工具

给多个大模型公益站（new-api / one-api 系面板）自动签到领奖，带本地 Web 统计面板。

已接入三个站点（均已实测跑通）：

| 站点 | 登录方式 | 领奖方式 | 反爬处理 |
| --- | --- | --- | --- |
| **VyceAI** (`vyceai.com`) | 邮箱密码（`.env`） | `POST /user/daily-reward/claim` | 自定义 API + 工作量证明 PoW（真浏览器自动过） |
| **AgentRouter** (`agentrouter.org`) | **GitHub 授权**（一次性手动） | 退出→重新登录触发 | 阿里云 WAF（余额从 localStorage 读，绕开拦截） |
| **JustWoker** (`api.justwoker.icu`) | **GitHub 授权**（一次性手动） | 访问 `/profile` 触发 | Cloudflare Turnstile（真浏览器自动过）+ token 鉴权 |

> 三个站机制各不相同（VyceAI 是自定义 API，另两个是 new-api/one-api 分支），工具已针对性适配。
> 奖励/余额优先从站点接口读取，读不到时从页面 localStorage 兜底。

---

## 🔒 隐私与安全（重要）

- **账号密码只存本地 `.env`**，已被 `.gitignore` 排除，**绝不入库、绝不上传**。
- 登录会话（Cookie）存在本地 `sessions/`，运行数据存在 `data/`，日志在 `logs/`，失败截图在 `screenshots/` —— **全部被 gitignore**。
- Web 面板默认**只监听 `127.0.0.1`（本机）**，不对局域网开放；如需远程访问请自行评估风险并设置 `PANEL_TOKEN`。
- 面板 API 不会返回任何账号密码。

> 提交前可用 `git status` 确认没有 `.env` / `config.json` / `data/` 等被跟踪。

---

## 快速开始

```bash
cd auto-checkin

# 1. 安装依赖
npm install

# 2. 安装 Playwright 的 Chromium 浏览器（首次必须）
npm run setup:browser

# 3. 配置密码类站点（VyceAI）
cp .env.example .env
# 编辑 .env，填入 VyceAI 的邮箱密码

# 4. 为 OAuth 类站点建立登录会话（AgentRouter / JustWoker，各跑一次）
#    会弹出浏览器，请在【弹出的那个窗口】里用 GitHub 登录一次（含 2FA）
npm run login -- agentrouter
npm run login -- justwoker

# 5.（可选）自定义运行配置
cp config.example.json config.json
# 编辑 config.json 改定时时间等。注意：OAuth/PoW/验证码需要真实浏览器，
# 建议 headless=false（若服务器无显示器可配合 xvfb）

# 6. 启动（常驻：Web 面板 + 每日定时签到）
npm start
```

## ⚠️ 关于 OAuth 登录（AgentRouter / JustWoker）

这两个站用 GitHub 授权登录，**不能用账号密码自动化**（也不应把 GitHub 密码交给工具）。方案是「一次性手动登录 + 会话复用」：

1. 跑 `npm run login -- <站点>`，工具打开一个浏览器窗口
2. **你在弹出的那个窗口里**用 GitHub 登录一次（你的 GitHub 密码只在官方 GitHub 页面输入，工具不接触、不存储）
3. 登录态存进本地 `sessions/<站点>/`（已 gitignore），之后每天自动签到复用，无需再登录
4. 若几周后 GitHub 会话过期，日志会提示 `请运行：npm run login -- <站点>`，重跑一次即可

> 登录检测会显示进度（每 15 秒一行）。**务必在工具弹出的窗口里登录**，不是你平时的浏览器——两者会话不通。

启动后打开面板：<http://127.0.0.1:8787>

---

## 运行模式

```bash
npm start                       # 常驻模式：启动面板 + 定时器（推荐）
npm run run-once                # 只跑一次全部站点后退出（供系统定时任务调用）
npm run run-site -- justwoker   # 只跑指定站点
npm run login -- agentrouter    # 为 OAuth 站点建立/刷新 GitHub 登录会话
npm test                         # 运行回归测试
```

面板上也可以点「立即全部签到」或每张卡片的「单独签到」手动触发。

Windows 计划任务应调用 `run-once.bat`。该脚本会先执行 `npm run setup:browser`，验证实际 headless 浏览器；只有明确发现 executable 缺失时才强制重装，再运行一次全部站点。包装层日志写入 `logs/task-wrapper.log`。浏览器安装/验证失败会写入 wrapper 日志并以非零退出；浏览器启动失败会按站点记录为失败，其他站点仍会继续。常驻面板使用独立 daemon 锁，实际签到、单站和手动登录共用 run 租约锁。已启用但缺少凭据的跳过也会使任务返回非零。

当前主机任务仍以 `Administrator` 的 InteractiveToken 运行；若要求“用户未登录也能执行”，需要另行改用服务账户/S4U 并重新注册任务，不能只靠批处理脚本解决。

---

## 配置说明（`config.json`）

```jsonc
{
  "schedule": {
    "cron": "0 9 * * *",          // 每天 9:00 触发，标准 cron 表达式
    "timezone": "Asia/Shanghai",
    "runOnStartup": true           // 启动时立即先跑一次
  },
  "browser": {
    "headless": true,              // 无头运行；调试选择器时可设 false 看浏览器
    "timeoutMs": 45000
  },
  "sites": {
    "vyceai": { "enabled": true },
    "agentrouter": { "enabled": true },
    "justwoker": { "enabled": true }
  }
}
```

面板地址/口令通过环境变量控制（写在 `.env`）：`PANEL_HOST`、`PANEL_PORT`、`PANEL_TOKEN`。

---

## 面板功能（2026 科技风）

玻璃拟态 + 极光背景 + Bento 网格，纯本地无外部依赖（图表为手绘 SVG）。

- **顶部指标 Bento**：今日已签/已配置、累计奖励、本周奖励、本月奖励、签到次数、成功率、平均耗时
- **奖励趋势折线图**：近 7/30/90 天每日领取奖励，各站分色 + 总额高亮，鼠标悬浮看每日明细
- **站点卡片（全字段）**：今日状态徽标、今日奖励、当前余额、累计领取、本周/本月、连续签到天数（+历史最长）、最近耗时（+均值）、成功率进度条、失败截图链接、最近说明
- **运行记录表**：最近 80 条，含时间/站点/状态/奖励/余额/耗时/说明
- 顶栏实时状态灯（运行中/下次定时时间）；自动每 15 秒刷新；支持手动触发单站或全部

---

## 关于站点适配（可能需要校准）

三个站都是 new-api/one-api 分支，共享 `/api/user/login`、`/api/user/self`、`/api/user/logout` 等接口，适配器据此实现。但各分支的**签到触发方式**可能有差异（有的有 `/api/user/check_in` 接口，有的靠访问页面）。

如果某站首次运行拿不到奖励：

1. 把 `config.json` 里 `browser.headless` 设为 `false`，`npm start` 观察浏览器实际行为；
2. 失败时看 `screenshots/` 里的截图和 `logs/` 里的日志；
3. 对应站点的触发逻辑在 `src/sites/<站点>.ts`，选择器和接口路径都在里面，按实际情况微调即可。

这部分是唯一可能需要你根据真实登录后页面微调的地方——框架、面板、统计、定时都是通用且已验证可跑的。

---

## 目录结构

```
auto-checkin/
├─ src/
│  ├─ index.ts          # 入口：解析参数，常驻/一次性/单站模式
│  ├─ config.ts         # 加载 .env 与 config.json，读取凭据
│  ├─ runner.ts         # 编排：逐站登录→领奖→读余额→落库
│  ├─ run-status.ts     # 一次性入口的失败退出码判定
│  ├─ verify-browser.ts # 计划任务启动前的浏览器可执行性验证
│  ├─ scheduler.ts      # node-cron 定时
│  ├─ server.ts         # Express 面板 API
│  ├─ store.ts          # 记录持久化 + 统计计算
│  ├─ browser.ts        # Playwright 持久化会话管理
│  ├─ lock.ts           # daemon 锁 + run 租约锁
│  ├─ logger.ts         # 控制台 + 按天落盘日志
│  ├─ login.ts          # OAuth 手动登录入口
│  ├─ sites/
│  │  ├─ newapi-client.ts  # new-api 通用客户端（登录/读余额/退出/签到）
│  │  ├─ vyceai.ts / agentrouter.ts / justwoker.ts
│  │  └─ index.ts          # 适配器注册表
│  └─ public/           # 面板前端（纯静态 HTML/CSS/JS）
├─ test/                # Node 内置测试：退出码与导航竞态
├─ run-once.bat         # Windows 计划任务入口（自动修复浏览器）
├─ .env.example         # 凭据模板（复制为 .env）
├─ config.example.json  # 配置模板（复制为 config.json）
└─ .gitignore           # 已排除所有密钥/会话/数据
```
