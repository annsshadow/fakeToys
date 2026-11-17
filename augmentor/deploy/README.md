# augmentor 测试环境部署

> 参照 `laiyipao/deploy/` 的子路径规范：统一域名 + 子路径，真值走环境变量/本地密钥文件（不入库）。
> 本目录只放模板与说明，**任何真实域名、口令、密钥都不得写进仓库**（见根 `AGENTS.md`）。

## 访问方式

- 前端：`http://<YOUR_TEST_DOMAIN>/augmentor/`（游客可读；写操作需口令）
- 健康检查：`http://<YOUR_TEST_DOMAIN>/augmentor-api/api/health`
- API：`http://<YOUR_TEST_DOMAIN>/augmentor-api/api`（nginx 剥离前缀后反代到容器 :8000）
- 账号密码：本平台**没有传统用户名/密码登录**。写操作（POST/PUT/PATCH/DELETE）走 `X-API-Key` 头，
  口令即环境变量 `AUGMENTOR_API_KEY` 的值，由部署环境注入；在前端「配置管理」页填入后存浏览器 localStorage。

## 真值来源（不入库）

| 变量 | 用途 | 来源 |
| --- | --- | --- |
| `DOMAIN` | 测试域名 | 本地 `.local.keys`（本仓约定，值不读不回显） |
| `AUGMENTOR_API_KEY` | 写鉴权口令 | 本地 `.local.keys` 或部署环境注入 |
| `VITE_BASE` | 前端子路径 | 构建时注入 `/augmentor/` |
| `VITE_API_BASE` | 前端 API 基址 | 构建时注入 `https://<YOUR_TEST_DOMAIN>/augmentor-api/api` |

`augmentor/deploy/.local.keys` 与 `.env` 必须在 `.gitignore` 中（本仓根 `.gitignore` 已拦
`.env` / `.env.*` / `*.env` / `*.local.keys` / `remote-config.env`），**真值文件只留在部署机**。

## 步骤 A：构建带子路径的前端产物

```bash
cd augmentor/web
VITE_BASE=/augmentor/ VITE_API_BASE=https://$DOMAIN/augmentor-api/api npm run build
npm run build:check      # 产物体检（已兼容子路径资源引用）
```

## 步骤 B：构建并启动后端容器（同源提供前端）

```bash
cd augmentor/docker
cp .env.example .env          # 真值只填本地 .env，不入库
# .env 中填 AUGMENTOR_API_KEY / 模型密钥；API_PORT=8000
docker compose up -d --build
curl http://127.0.0.1:8000/api/health
```

## 步骤 C：挂到测试域名子路径

1. 将 `augmentor/deploy/nginx/augmentor-test.conf` 拷到 `/etc/nginx/conf.d/augmentor/10-test.conf`。
2. 在 home.conf 的 server 块内加 `include /etc/nginx/conf.d/augmentor/*.conf;`。
3. 把 `<远端前端目录>` 替换为真实部署目录（部署机上改，不入库）。
4. `nginx -t && nginx -s reload`。

> 前端 `dist` 由后端容器在根路径同源提供时，也可不用 nginx 静态别名，直接用 location /augmentor/ 反代到容器根路径；
> 上述片段是「静态资源由 nginx 直接服务、API 反代」的推荐形态（与 laiyipao 一致）。
