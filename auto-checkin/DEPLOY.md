# auto-checkin 计划任务部署手册（换机器用）

> 用途：把 `auto-checkin` 的 Windows 计划任务在**另一台机器**上完整复刻。
> 本文所有参数均从已跑通的实机（`PC-20260417JOSA`）导出，不是推测值。
> 生成日期：2026-10-13

---

## 0. 前置条件

| 项 | 要求 | 说明 |
| --- | --- | --- |
| Node.js | **≥ 20** | `run-once.bat` 会主动探测，低于 20 直接跳过该候选 |
| npm | 任意随 Node 附带版本 | 脚本会逐个候选校验，不信任第一个 PATH 命中 |
| Playwright Chromium | 由脚本自动装 | 每次任务先 `setup:browser` 再 `verify:browser` |
| 仓库路径 | 任意 | 本机为 `D:\WORKSPACE\fakeToys\auto-checkin`，**不硬编码**在任务里，用 `%~dp0` |

代码仓库已包含全部脚本，**换机器只需复制 `auto-checkin/` 目录 + 建计划任务**。

---

## 1. 首次部署（在目标机器上执行一次）

```powershell
cd <auto-checkin 目录>
npm install
npm run setup:browser        # 装 Chromium
npm run verify:browser       # 验证实际 headless executable，退出码 20 = 缺失
```

### 1.1 配置密码类站点

```powershell
Copy-Item .env.example .env
# 编辑 .env，至少填 VYCEAI_EMAIL / VYCEAI_PASSWORD
```

`.env`、`config.json`、`sessions/`、`data/`、`logs/`、`screenshots/` **全部已在 `.gitignore` 中**，换机器时不会被 git 带过去，需要各自重建。

### 1.2 建立 OAuth 站点会话（各跑一次，必须在弹出的窗口里登录）

```powershell
npm run login -- justwoker
npm run login -- agentrouter --checkin
```

- **JustWoker**：Cloudflare Turnstile，真浏览器自动过。
- **AgentRouter**：余额接口 `/api/user/self` 被阿里云 WAF 单点保护，会弹**滑块人机验证**。
  工具不做自动绕过（反爬绕过不在本项目范围），`--checkin` 的作用是：
  **登录 → 等你手动拖滑块 → 在同一个浏览器上下文里立即签到**。
  WAF 放行窗口只有十几秒，拆成"先 login 再 run-site"两步会来不及，所以必须带 `--checkin`。
- GitHub 密码只在官方页面输入，工具不接触、不存储。

### 1.3 可选：运行配置

```powershell
Copy-Item config.example.json config.json
```

```json
{
  "schedule": { "cron": "0 9 * * *", "timezone": "Asia/Shanghai", "runOnStartup": true },
  "browser":  { "headless": true, "timeoutMs": 45000 },
  "sites":    { "vyceai": {"enabled":true}, "agentrouter": {"enabled":true}, "justwoker": {"enabled":true} }
}
```

⚠️ `config.json` 只影响**常驻面板模式**（`npm start`）。计划任务走 `--once`，不读 cron。

---

## 2. 注册计划任务

### 2.1 实机导出的任务定义（原样照抄，只改路径）

```powershell
Export-ScheduledTask -TaskName AutoCheckin > AutoCheckin.xml
```

XML 全文：

```xml
<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Date>2026-09-21T17:18:42</Date>
    <Author>PC-20260417JOSA\Administrator</Author>
    <URI>\AutoCheckin</URI>
  </RegistrationInfo>
  <Principals>
    <Principal id="Author">
      <UserId>S-1-5-21-1970804692-4136476608-1797970276-500</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <ExecutionTimeLimit>PT2H</ExecutionTimeLimit>
    <MultipleInstancesPolicy>Queue</MultipleInstancesPolicy>
    <RestartOnFailure>
      <Count>3</Count>
      <Interval>PT5M</Interval>
    </RestartOnFailure>
    <StartWhenAvailable>true</StartWhenAvailable>
    <IdleSettings>
      <Duration>PT10M</Duration>
      <WaitTimeout>PT1H</WaitTimeout>
      <StopOnIdleEnd>true</StopOnIdleEnd>
      <RestartOnIdle>false</RestartOnIdle>
    </IdleSettings>
  </Settings>
  <Triggers>
    <CalendarTrigger>
      <StartBoundary>2026-09-21T09:00:00</StartBoundary>
      <ScheduleByDay>
        <DaysInterval>1</DaysInterval>
      </ScheduleByDay>
    </CalendarTrigger>
  </Triggers>
  <Actions Context="Author">
    <Exec>
      <Command>cmd.exe</Command>
      <Arguments>/c "&lt;auto-checkin 绝对路径&gt;\run-once.bat"</Arguments>
    </Exec>
  </Actions>
</Task>
```

### 2.2 换机器必须改的两处

| 字段 | 改成 |
| --- | --- |
| `UserId` | 目标机器上运行任务的用户 SID（`whoami /user` 可查） |
| `Actions/Exec/Arguments` | 目标机器上 `run-once.bat` 的绝对路径 |

`StartBoundary` 改成目标机器期望的首次执行时间。

### 2.3 注册命令

```powershell
# 先改好 XML 里的上面两处
Register-ScheduledTask -TaskName AutoCheckin -Xml (Get-Content .\AutoCheckin.xml -Raw) -Force
```

---

## 3. 逐项说明这些设置为什��这么设

| 设置 | 值 | 原因 |
| --- | --- | --- |
| `MultipleInstancesPolicy` | `Queue` | 默认 `IgnoreNew` 会让 09:00 的任务在补跑还在跑时被**静默丢弃**。队列化才能排队执行。 |
| `StartWhenAvailable` | `true` | 机器关机/睡眠错过 09:00 时，开机后补跑。**实测有效但有边界**：连续错过 3 天（10/04–10/06）时并未补跑，需确认目标机器不会长期离线。 |
| `RestartOnFailure` | 3 次 / 5 分钟 | 浏览器或网络瞬时失败时自动重来。 |
| `ExecutionTimeLimit` | `PT2H` | 默认 72 小时太长，卡死的进程会一直占着 run 锁。 |
| `DisallowStartIfOnBatteries` / `StopIfGoingOnBatteries` | `false` | 笔记本接电/用电池都能跑。 |
| `RunLevel` | `HighestAvailable` | 避免权限不足导致浏览器起不来。 |
| `LogonType` | `InteractiveToken` | 见下方"已知限制"。 |

### 已知限制（必须让使用者知道）

**`InteractiveToken` 意味着用户未登录时任务不会运行。**

本机实测：10/04–10/06 连续三天未执行（`NumberOfMissedRuns=3`），因为机器处于未登录状态。

若要求"无人登录也能跑"，需要：
1. 创建专用服务账户（不用日常账户）
2. 把 `LogonType` 改为 `S4U`，或改用服务账户 + 存储密码
3. **重新注册任务**（改账户类型不能只靠改批处理脚本解决）
4. 重新验证：`Start-ScheduledTask` 后确认 `LastTaskResult`

---

## 4. 任务入口脚本做了什么

`run-once.bat`（**必须保持纯 ASCII**，cmd.exe 在 GBK 下会误解析 UTF-8 中文）：

```
1. 建日志通道 → logs/task-wrapper.log（建不了就退化到 %TEMP%）
2. cd /d %~dp0，失败即 exit 1
3. 逐个校验 npm 候选：nvm 固定路径 → %APPDATA%\npm → %ProgramFiles%\nodejs → where npm
   每个候选都要求：node.exe 存在、--version 能跑、主版本号 ≥ 20
   全部失败 → exit 9009
4. npm run setup:browser      （幂等，已存在则空操作）
5. npm run verify:browser
   exit 20 → 判定 executable 缺失 → npm run repair:browser → 再 verify
   其它非 0 → 直接失败，不浪费一次重装
6. npm run run-once           （跑全部站点）
7. 透传 run-once 的退出码
```

**为什么要这层包装**：实测踩过的坑——Playwright 缓存被清后任务仍"执行成功"却什么都没跑（退出码 1 但日志不显眼）；
以及 `node-cron` 依赖漂移。`verify:browser` 校验的是**实际 headless 可执行文件**，不是 Playwright 的安装标记文件。

---

## 5. 锁与并发模型

两个独立锁（`src/lock.ts`，基于 `proper-lockfile` 的原子 mkdir + mtime 心跳）：

| 锁 | 谁持有 | 目的 |
| --- | --- | --- |
| `.daemon.lock` | `npm start` 常驻面板 | 单实例 |
| `.run.lock` | 实际签到 / 单站 / 手动登录 | 浏览器 profile 互斥 |

**为什么拆两个**：早期用单锁导致常驻面板存在时会**阻断计划任务**，09:00 静默不跑。
拆分后面板常驻不再影响一次性任务。

- stale 窗口 **5 分钟**，心跳 10 秒
- 进程被强杀会留下锁，需等 5 分钟过期或确认 PID 已死后手动删 `sessions/.run.lock`
- 锁目录旁有 `.run.lock.owner`（JSON，含 pid + token）用于报出占用者

---

## 6. 退出码语义（排障时必看）

| 退出码 | 含义 |
| --- | --- |
| `0` | 全部站点 `success` / `already` / 已显式禁用 |
| `1` | 任一站点 `failed`，**或**已启用但缺凭据被 `skipped` |
| `20` | `verify:browser` 专用：headless executable 缺失 |
| `9009` | 没找到可用的 Node ≥ 20 |

**关键设计：宁可失败也不误报成功。** 领取前后余额读不到时一律判 `failed`，
不把未知状态写成 `already` 或 `success`——否则会静默丢奖励而无人察觉。

---

## 7. 换机器后的验收清单

```powershell
# 1. 任务能手动触发
Start-ScheduledTask -TaskName AutoCheckin

# 2. 等它跑完，确认结果
Get-ScheduledTaskInfo -TaskName AutoCheckin |
  Select-Object LastRunTime, LastTaskResult, NextRunTime

# 3. LastTaskResult 必须是 0；非 0 就翻日志
Get-Content <auto-checkin>\logs\task-wrapper.log -Tail 40

# 4. 锁已释放（不应残留）
Get-ChildItem <auto-checkin>\sessions -Force | Where-Object Name -match 'lock'

# 5. 三站结果逐条核对
Get-Content <auto-checkin>\data\records.json -Tail 60
```

首次验收时 `LastTaskResult` 很可能非 0，**这是预期行为**：AgentRouter 需要人工过滑块，
无人值守跑必然失败。确认其余两站 `success`/`already` 即可视为部署成功。

---

## 8. 常见故障速查

| 现象 | 原因 | 处理 |
| --- | --- | --- |
| 任务整点没动静，`NumberOfMissedRuns` 增长 | `InteractiveToken` + 用户未登录 | 见 §3 已知限制 |
| 日志报 `wrapper:no_working_npm` | 无 Node ≥ 20 | 装 Node 20+ |
| 日志报 `browser_verify_failed; forcing reinstall` | Playwright 缓存被清 | 脚本会自动重装；持续失败则手动 `npm run repair:browser` |
| 报"已有其他签到/登录进程在运行（PID=xxx）" | 上次进程被强杀，锁未过期 | 确认 PID 已死后删 `sessions/.run.lock`，或等 5 分钟 |
| `LastTaskResult=3221225786`（0xC000013A） | 任务控制台被 Ctrl+C 人工终止，非脚本错误 | 属正常中断语义；清掉残留锁后重新触发即可 |
| vyceai 账号密码已填，但登录页停在原状最终 failed | 站点按浏览器 locale/IP 输出中文 UI，登录按钮可访问名是「登录」而非 "Login"，点击选择器不命中被 catch 吞掉 | 已在 `src/sites/vyceai.ts` 修复为双语兼容 `/^(Login|登录)$/`；若日志出现「未找到登录按钮…」说明页面结构又变了 |
| Chromium 启动即崩溃，退出码 `0xC00000FD` | **系统提交内存耗尽**，非代码问题 | 关掉占内存的大进程（实测 MuMu 模拟器 6.9 GB、Tabbit 2.8 GB）后重试 |
| AgentRouter 报 WAF 人机验证 | 站点反爬，余额接口被拦 | `npm run login -- agentrouter --checkin`，手动拖滑块 |

---

## 9. 代码版本

本文对应的代码提交：`f81dd09c`（WAF 挑战页识别 / OAuth 弹窗接管 / 放行 cookie 保留）。
更早的加固提交：`298c25b4f`（浏览器自愈、退出码语义、双锁拆分）。