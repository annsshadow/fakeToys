# 03 · 客户端 / Web 管理端（`web/`）

> 客户端是一个**纯浏览器 SPA**（React 19 + TypeScript + Vite 8 + Ant Design 6 + ECharts），
> 不持有任何自有数据与 LLM——所有能力都通过 `axios` 调用**服务端 REST**（`baseURL: '/api'`）。
> 它是平台唯一的图形化入口；**没有移动端**对应物。

---

## 1. 整体架构

```
浏览器 (web/dist 或 :3000 dev server)
   │
   │  axios (baseURL "/api", 30s 默认超时, 写操作拦截器附 X-API-Key)
   ▼
服务端 FastAPI (api/)   ◄──── dev 时 vite 把 /api 代理到 localhost:8000
```

- **同源部署**：`npm run build` 产出 `web/dist`，由 `api/main.py` 用 `StaticFiles(html=True)`
  挂载在 `/`。此时前端与后端**同源**，`axios` 走相对路径 `/api`，**不需要 CORS**。
- **开发部署**：`npm run dev`（:3000），`vite.config.ts` 把 `/api` 代理到 `http://localhost:8000`。
- **跨源部署**（前端跑在别的站点）：需在后端 `web.cors_origins` 显式白名单来源
  （出厂默认 `[]` = 不发任何 CORS 头，见 `config.yaml` 注释）。

---

## 2. 页面划分（11 个功能页）

由 `web/src/App.tsx` 的 `<Routes>` 注册（`lazy` 懒加载 + `Suspense` 兜底）：

| 路由 | 页面 | 组件 | 覆盖能力（对应后端 tag） |
|------|------|------|--------------------------|
| `/dashboard` | 仪表盘 | `Dashboard` | 健康状态、模型清单、版本快照、快速增强入口 |
| `/` | 数据管理 | `DataManagement` | 文件列表/分页/搜索/增删改、上传、按格式导出、格式转换、合并、分割、采样、多源聚合、RAG、批量翻译 |
| `/augment` | 数据增强 | `Augmentation` | 启动增强任务、进度轮询、断点列表 |
| `/quality` | 质量中心 | `Quality` | 评估/去重/报告/清洗/标注/基准/离群/画像/结构校验/健康门禁 |
| `/security` | 数据安全 | `Security` | PII 脱敏 / 泄漏检测 / 就绪审计 |
| `/export` | 导出中心 | `Export` | 格式清单、预览、批量导出 |
| `/analysis` | 数据分析 | `Analysis` | 覆盖度、统计、可视化、语言画像、生成集对比、内容/语义检索、增强前后影响、特征普查、双集对比 |
| `/multimodal` | 多模态 | `Multimodal` | 单条处理、目录扫描 |
| `/versions` | 版本管理 | `Versions` | 列表、对比、回滚、历史 |
| `/settings` | 配置管理 | `Settings` | 配置读写、模型清单、阈值推荐、配置校验、**写操作鉴权（存 `X-API-Key`）** |
| `/system` | 系统状态 | `System` | 综合状态、依赖明细、降级功能、向量库、备份、数据集血缘、质量监控/自测、迁移/流式 |

侧边栏 `Menu` 与路由一一对应；标题统一为「企业级 AI 训练数据平台」。

---

## 3. 模块划分（前端工程）

```
web/
├── package.json / vite.config.ts / tsconfig.json / eslint / vitest
├── index.html                       SPA 入口（<div id="root"> + src/main.tsx）
├── scripts/check-dist.mjs           build:check 后的产物校验
└── src/
    ├── main.tsx                    根组件 + antd ConfigProvider
    ├── App.tsx                     Layout + 11 路由注册（lazy）
    ├── index.css                   全局样式
    ├── pages/                     ★ 11 个功能页（每页配同名 *.test.tsx）
    ├── components/                复用组件（AugmentForm / DataList / ExportDialog /
    │                               QualityChart / VersionTimeline）
    ├── services/
    │   ├── api.ts                 ★ axios 实例 + 全部后端封装 + 写操作鉴权拦截器
    │   └── api.wiring.test.ts     ★ 接线守门测试
    ├── types/api.ts               ★ 与后端 77 端点对齐的 TS 类型（~80 个响应接口）
    └── test/setup.ts              vitest 环境
```

---

## 4. 服务层 `services/api.ts`（客户端 ↔ 服务端 的唯一通道）

- `const api = axios.create({ baseURL: '/api', timeout: 30000 })`。
- **写操作鉴权**：浏览器在 `localStorage['augmentor.api_key']` 存一份 key，
  请求拦截器随每个请求附 `X-API-Key`（留空则不发该头，保持零配置可用）。
  > 这是明文、单机、非保密措施——解决「服务端开了鉴权时页面还能用」，
  > 强保护需配后端会话/CSRF，超出此格范围。
- 导出的每个后端封装函数对应一个/一组 REST 端点（类型来自 `types/api.ts`）。

### 4.1 接线守门测试（重要质量门禁）

`services/api.wiring.test.ts` 有一条**接线守门**：`api.ts` 导出的每个函数
**必须**在某个页面源码里被真正引用（`import` 与注释都不算）。
「加了封装却忘了接页面」这类漏法，类型检查 / lint / 覆盖率都发现不了，
靠这条测试兜底。**新增 `api.ts` 函数时务必对应接进页面，否则该测试变红。**

---

## 5. 关键接口与数据结构（客户端视角）

客户端消费的服务端端点全量见 [`02-server.md § 4`](02-server.md) 与 [`API.md`](../API.md)。
`web/src/types/api.ts` 定义了约 80 个响应接口（`HealthResponse` / `DataPageResponse` /
`QualityEvaluationResponse` / `VersionDetail` / `BackupInfo` / `DependencyGraphResponse` …），
与后端 Pydantic `response_model` 逐字段对齐。

**读侧分页的关键契约**：`GET /api/data/load/{filename}` 返回的 `indices` 与 `items` 同长同序，
逐格是「这一条在盘上文件里的 0 基下标」——它是 `PUT /api/data/update` 与 `DELETE /api/data/delete`
唯一的权威坐标（这两端读整份文件按 `items[index]` 写盘，**不能**用页内下标）。
`DataManagement.tsx` 据此实现编辑/删除。

---

## 6. 与后端的交互关系

| 场景 | 前端动作 | 服务端端点 | 说明 |
|------|----------|------------|------|
| 列表 | 拉文件清单 | `GET /api/data/list` | 取候选 |
| 分页读 | 翻页/搜索 | `GET /api/data/load/{f}` | 携 `page/page_size/search`，回 `items` + `indices` |
| 改/删单条 | 用 `indices` 定位 | `PUT/DELETE /api/data/…` | **写操作**，设 key 时附 `X-API-Key` |
| 增强 | 点「启动增强」 | `POST /api/augment/start` | 轮询 `GET /api/augment/progress` |
| 批量翻译 | 勾选确认后点 | `POST /api/dataset/translate` | 逐条调模型，按 token 计费 |
| 存 key | Settings 页 | 仅写 `localStorage` | 不进构建产物、不进仓库 |

> 完整前端↔后端流程见 [`05-interactions-and-dataflow.md`](05-interactions-and-dataflow.md)。

---

## 7. 目录结构与脚本

见 § 3。常用脚本（`web/package.json`）：

| 脚本 | 作用 |
|------|------|
| `npm run dev` | Vite 开发服务器（:3000，`/api` 代理 :8000） |
| `npm run build` | `tsc --noEmit && vite build` → `web/dist` |
| `npm run build:check` | build 后跑 `scripts/check-dist.mjs` 校验产物 |
| `npm run lint` | `eslint . --max-warnings 0` |
| `npm test` | `vitest run`（含接线守门与 11 页的测试） |
| `npm run format` | Prettier |
